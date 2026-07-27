// Site behavior for the VB6 workspace docs.
(function() {
    async function loadProjects() {
        try {
            const response = await fetch('assets/data/projects.json', { cache: 'no-store' });
            if (!response.ok) {
                throw new Error('Project metadata not available');
            }

            return await response.json();
        } catch (error) {
            const message = document.getElementById('project-load-message');
            if (message) {
                message.textContent = 'Project metadata could not be loaded from assets/data/projects.json. Serve the docs folder over HTTP to view the full hub.';
                message.classList.add('warning-box');
                message.style.display = 'block';
            }

            return [];
        }
    }

    function statusClass(status) {
        if (status === 'stable') {
            return 'status-stable';
        }

        if (status === 'support') {
            return 'status-support';
        }

        if (status === 'planning') {
            return 'status-planning';
        }

        if (status === 'design') {
            return 'status-design';
        }

        return 'status-development';
    }

    function renderProjectCards(projects) {
        const grid = document.getElementById('project-grid');
        if (!grid) {
            return;
        }

        if (!projects.length) {
            grid.innerHTML = '<div class="info-card"><h3>Project metadata unavailable</h3><p>The hub expects <code>assets/data/projects.json</code> to be available. Serve the docs folder through a local web server or deploy it to GitHub Pages.</p></div>';
            return;
        }

        grid.innerHTML = projects.map((project) => `
            <article class="project-card">
                <div class="project-card-top">
                    <div>
                        <p class="project-category">${project.category}</p>
                        <h3>${project.name}</h3>
                    </div>
                    <span class="status-pill ${statusClass(project.status)}">${project.statusLabel}</span>
                </div>
                <p class="project-summary">${project.summary}</p>
                <p class="project-detail">${project.statusDetail}</p>
                <ul class="project-notes">
                    ${project.notes.map((note) => `<li>${note}</li>`).join('')}
                </ul>
                <div class="card-actions">
                    <a class="button button-primary" href="${project.repoUrl}" target="_blank" rel="noreferrer">Source</a>
                    ${project.docsUrl ? `<a class="button button-secondary" href="${project.docsUrl}" target="_blank" rel="noreferrer">Docs</a>` : ''}
                </div>
            </article>
        `).join('');

        const projectCount = document.getElementById('project-count');
        const stableCount = document.getElementById('stable-count');
        const docsCount = document.getElementById('docs-count');

        if (projectCount) {
            projectCount.textContent = String(projects.length);
        }

        if (stableCount) {
            stableCount.textContent = String(projects.filter((project) => project.status === 'stable').length);
        }

        if (docsCount) {
            docsCount.textContent = String(projects.filter((project) => Boolean(project.docsUrl)).length);
        }
    }

    function renderStatusTable(projects) {
        const body = document.getElementById('status-table-body');
        if (!body) {
            return;
        }

        if (!projects.length) {
            body.innerHTML = '<tr><td colspan="4">No project metadata is available yet.</td></tr>';
            return;
        }

        body.innerHTML = projects.map((project) => `
            <tr>
                <td>
                    <div class="table-project-name">${project.name}</div>
                    <div class="table-project-category">${project.category}</div>
                </td>
                <td><span class="status-pill ${statusClass(project.status)}">${project.statusLabel}</span></td>
                <td>${project.statusDetail}</td>
                <td>
                    <ul class="table-notes">
                        ${project.notes.map((note) => `<li>${note}</li>`).join('')}
                    </ul>
                </td>
            </tr>
        `).join('');

        const allCount = document.getElementById('all-count');
        const activeCount = document.getElementById('active-count');
        const docsReadyCount = document.getElementById('docs-ready-count');

        if (allCount) {
            allCount.textContent = String(projects.length);
        }

        if (activeCount) {
            activeCount.textContent = String(projects.filter((project) => project.status !== 'stable').length);
        }

        if (docsReadyCount) {
            docsReadyCount.textContent = String(projects.filter((project) => Boolean(project.docsUrl)).length);
        }
    }

    async function init() {
        const projects = await loadProjects();
        renderProjectCards(projects);
        renderStatusTable(projects);
    }

    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', init);
    } else {
        init();
    }
})();