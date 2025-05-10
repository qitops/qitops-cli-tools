// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item affix "><a href="index.html">QitOps: The Ultimate QA CLI Tool</a></li><li class="chapter-item affix "><li class="part-title">User Guide</li><li class="chapter-item "><a href="index.html"><strong aria-hidden="true">1.</strong> Why QitOps?</a></li><li class="chapter-item "><a href="index.html"><strong aria-hidden="true">2.</strong> Installation</a></li><li class="chapter-item "><a href="index.html"><strong aria-hidden="true">3.</strong> Quick Start</a></li><li class="chapter-item affix "><li class="part-title">Core Features</li><li class="chapter-item "><a href="index.html"><strong aria-hidden="true">4.</strong> API Testing</a></li><li class="chapter-item "><a href="index.html"><strong aria-hidden="true">5.</strong> API Collections</a></li><li class="chapter-item "><a href="index.html"><strong aria-hidden="true">6.</strong> Performance Testing</a></li><li class="chapter-item "><a href="index.html"><strong aria-hidden="true">7.</strong> Security Testing</a></li><li class="chapter-item "><a href="index.html"><strong aria-hidden="true">8.</strong> Web Testing</a></li><li class="chapter-item affix "><li class="part-title">AI-Powered Testing</li><li class="chapter-item "><a href="ai-features.html"><strong aria-hidden="true">9.</strong> AI Features Overview</a></li><li class="chapter-item "><a href="ai-features.html"><strong aria-hidden="true">10.</strong> Test Generation</a></li><li class="chapter-item "><a href="ai-features.html"><strong aria-hidden="true">11.</strong> Results Analysis</a></li><li class="chapter-item "><a href="ai-features.html"><strong aria-hidden="true">12.</strong> Improvement Suggestions</a></li><li class="chapter-item "><a href="ai-features.html"><strong aria-hidden="true">13.</strong> Local LLM Support</a></li><li class="chapter-item "><a href="testing-ai-features.html"><strong aria-hidden="true">14.</strong> Testing AI Features</a></li><li class="chapter-item affix "><li class="part-title">Using QitOps</li><li class="chapter-item "><a href="usage.html"><strong aria-hidden="true">15.</strong> Usage Guide</a></li><li class="chapter-item "><a href="configuration.html"><strong aria-hidden="true">16.</strong> Configuration</a></li><li class="chapter-item "><a href="best-practices.html"><strong aria-hidden="true">17.</strong> Best Practices</a></li><li class="chapter-item "><a href="index.html"><strong aria-hidden="true">18.</strong> CI/CD Integration</a></li><li class="chapter-item "><a href="index.html"><strong aria-hidden="true">19.</strong> Data-Driven Testing</a></li><li class="chapter-item affix "><li class="part-title">Project Information</li><li class="chapter-item "><a href="../ROADMAP.html"><strong aria-hidden="true">20.</strong> Roadmap</a></li><li class="chapter-item "><a href="../CHANGELOG.html"><strong aria-hidden="true">21.</strong> Changelog</a></li><li class="chapter-item "><a href="../CONTRIBUTING.html"><strong aria-hidden="true">22.</strong> Contributing</a></li><li class="chapter-item "><a href="index.html"><strong aria-hidden="true">23.</strong> License</a></li><li class="chapter-item affix "><li class="spacer"></li><li class="chapter-item affix "><a href="GITHUB_PAGES.html">Documentation Setup</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
