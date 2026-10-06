import { marked } from 'marked';

const pages = ['DESIGN', 'CHECKLIST', 'PROGRESS', 'DECISIONS', 'PARITY'];
const side = document.getElementById('side');
side.innerHTML = pages
  .map((p) => `<a href="/docs.html?page=${p}">${p}</a>`)
  .join('');

const params = new URLSearchParams(location.search);
const page = (params.get('page') || 'DESIGN').toUpperCase();
fetch(`/docs/${page}.md`)
  .then((r) => (r.ok ? r.text() : Promise.reject(r.status)))
  .then((md) => {
    document.getElementById('doc').innerHTML = marked.parse(md);
  })
  .catch(() => {
    document.getElementById('doc').innerHTML = `<h1>Doc not published yet</h1><p>${page}.md is not in /docs yet.</p>`;
  });
