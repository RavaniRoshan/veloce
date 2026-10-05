//! The reference API from the design doc, adapted: `k8s::stream_deploy_logs`
//! is replaced with a mock stream so the example compiles standalone.

use anyhow::Result;

use veloce::prelude::*;

#[derive(Default, Debug, Clone)]
pub struct DeploymentDashboard {
    service_name: String,
    status: DeploymentStatus,
    logs: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub enum DeploymentStatus {
    #[default]
    Idle,
    Deploying,
    Live,
}

impl DeploymentStatus {
    fn color(&self) -> ratatui::style::Color {
        match self {
            DeploymentStatus::Idle => ratatui::style::Color::Gray,
            DeploymentStatus::Deploying => ratatui::style::Color::Yellow,
            DeploymentStatus::Live => ratatui::style::Color::Green,
        }
    }
}

#[derive(Clone)]
pub enum Action {
    DeployRequested,
    LogReceived(String),
    DeployComplete,
}

#[async_trait::async_trait]
impl View for DeploymentDashboard {
    type Action = Action;

    async fn load(&mut self) -> Result<()> {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        self.service_name = "auth-service-v2".into();
        self.status = DeploymentStatus::Idle;
        Ok(())
    }

    fn update(&mut self, action: Self::Action, cx: &mut Context<Self::Action>) {
        match action {
            Action::DeployRequested => {
                self.status = DeploymentStatus::Deploying;
                let tx = cx.dispatcher(); // accessor per the resolved contract
                cx.spawn(async move {
                    let mut i = 0u32;
                    while i < 5 {
                        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                        tx.dispatch(Action::LogReceived(format!("pod log line {i}")));
                        i += 1;
                    }
                    tx.dispatch(Action::DeployComplete);
                });
            }
            Action::LogReceived(line) => self.logs.push(line),
            Action::DeployComplete => self.status = DeploymentStatus::Live,
        }
    }

    fn view(&self) -> Element {
        Flex::column()
            .gap(1)
            .child(
                Flex::row()
                    .padding(1)
                    .border(BorderStyle::Rounded)
                    .child(Text::new(format!("Service: {}", self.service_name)).bold())
                    .child(Spacer::grow())
                    .child(
                        Text::new(format!("Status: {:?}", self.status)).color(self.status.color()),
                    ),
            )
            .child(
                ScrollView::new()
                    .flex_grow(1.0)
                    .children(self.logs.iter().map(|l| Text::new(l).into_element())),
            )
            .into_element()
    }

    fn handle_key(
        &mut self,
        key: crossterm::event::KeyEvent,
        cx: &mut Context<Self::Action>,
    ) -> bool {
        use crossterm::event::KeyCode;
        match key.code {
            KeyCode::Char('d') => {
                cx.dispatch(Action::DeployRequested);
                true
            }
            _ => false,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    VeloceApp::new()
        .route("/", DeploymentDashboard::default())
        .run()
        .await
}
