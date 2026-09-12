// Theme switcher for the VB6 workspace docs.
(function() {
    const THEME_KEY = 'vb6-workspace-theme';
    const HIGHLIGHT_CSS_URLS = {
        light: 'https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/github.min.css',
        dark: 'https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/github-dark.min.css'
    };
    const HIGHLIGHT_CSS_ID = 'hljs-theme';

    function getSystemPreference() {
        if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
            return 'dark';
        }

        return 'light';
    }

    function getTheme() {
        const savedTheme = localStorage.getItem(THEME_KEY);
        if (savedTheme) {
            return savedTheme;
        }

        return getSystemPreference();
    }

    function saveTheme(theme) {
        localStorage.setItem(THEME_KEY, theme);
    }

    function applyHighlightTheme(theme) {
        let linkEl = document.getElementById(HIGHLIGHT_CSS_ID);
        if (!linkEl) {
            linkEl = document.createElement('link');
            linkEl.id = HIGHLIGHT_CSS_ID;
            linkEl.rel = 'stylesheet';
            document.head.appendChild(linkEl);
        }
        linkEl.href = HIGHLIGHT_CSS_URLS[theme] || HIGHLIGHT_CSS_URLS.dark;

        if (window.hljs) {
            window.hljs.highlightAll();
        }
    }

    function applyTheme(theme) {
        document.documentElement.setAttribute('data-theme', theme);

        const icon = document.querySelector('.theme-icon');
        if (icon) {
            icon.textContent = theme === 'dark' ? '☀️' : '🌙';
        }

        applyHighlightTheme(theme);
    }

    function toggleTheme() {
        const nextTheme = getTheme() === 'dark' ? 'light' : 'dark';
        saveTheme(nextTheme);
        applyTheme(nextTheme);
    }

    const initialTheme = getTheme();
    document.documentElement.setAttribute('data-theme', initialTheme);

    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', function() {
            applyTheme(initialTheme);
            const toggleButton = document.getElementById('theme-toggle');
            if (toggleButton) {
                toggleButton.addEventListener('click', toggleTheme);
            }
        });
    } else {
        applyTheme(initialTheme);
        const toggleButton = document.getElementById('theme-toggle');
        if (toggleButton) {
            toggleButton.addEventListener('click', toggleTheme);
        }
    }

    if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').addEventListener) {
        window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', function(e) {
            const savedTheme = localStorage.getItem(THEME_KEY);
            if (!savedTheme) {
                applyTheme(e.matches ? 'dark' : 'light');
            }
        });
    }
})();
