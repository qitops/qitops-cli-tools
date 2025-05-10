// QitOps Documentation Custom JavaScript

document.addEventListener('DOMContentLoaded', function() {
    // Add version selector
    addVersionSelector();
    
    // Add copy buttons to code blocks
    addCopyButtons();
    
    // Add anchor links to headings
    addAnchorLinks();
    
    // Add table of contents for current page
    addTableOfContents();
});

function addVersionSelector() {
    // This is a placeholder for future version selector functionality
    // When multiple versions of the documentation are available, this can be implemented
}

function addCopyButtons() {
    // Add copy button to code blocks
    const codeBlocks = document.querySelectorAll('pre');
    
    codeBlocks.forEach(function(codeBlock) {
        if (codeBlock.querySelector('code')) {
            const copyButton = document.createElement('button');
            copyButton.className = 'copy-button';
            copyButton.textContent = 'Copy';
            
            copyButton.addEventListener('click', function() {
                const code = codeBlock.querySelector('code').textContent;
                navigator.clipboard.writeText(code).then(function() {
                    copyButton.textContent = 'Copied!';
                    setTimeout(function() {
                        copyButton.textContent = 'Copy';
                    }, 2000);
                }, function(err) {
                    console.error('Could not copy text: ', err);
                    copyButton.textContent = 'Error!';
                    setTimeout(function() {
                        copyButton.textContent = 'Copy';
                    }, 2000);
                });
            });
            
            codeBlock.style.position = 'relative';
            copyButton.style.position = 'absolute';
            copyButton.style.top = '5px';
            copyButton.style.right = '5px';
            copyButton.style.padding = '3px 8px';
            copyButton.style.border = 'none';
            copyButton.style.borderRadius = '3px';
            copyButton.style.background = '#4a86e8';
            copyButton.style.color = 'white';
            copyButton.style.fontSize = '0.8em';
            copyButton.style.cursor = 'pointer';
            
            codeBlock.appendChild(copyButton);
        }
    });
}

function addAnchorLinks() {
    // Add anchor links to headings
    const headings = document.querySelectorAll('h1, h2, h3, h4, h5, h6');
    
    headings.forEach(function(heading) {
        if (heading.id) {
            const anchor = document.createElement('a');
            anchor.className = 'anchor-link';
            anchor.href = '#' + heading.id;
            anchor.textContent = '#';
            
            anchor.style.marginLeft = '0.5em';
            anchor.style.opacity = '0.5';
            anchor.style.textDecoration = 'none';
            anchor.style.fontSize = '0.8em';
            anchor.style.verticalAlign = 'middle';
            
            heading.appendChild(anchor);
        }
    });
}

function addTableOfContents() {
    // Add table of contents for current page
    const content = document.querySelector('.content');
    const headings = document.querySelectorAll('h2, h3');
    
    if (headings.length > 3) {
        const toc = document.createElement('div');
        toc.className = 'table-of-contents';
        toc.innerHTML = '<h2>Table of Contents</h2><ul></ul>';
        
        const tocList = toc.querySelector('ul');
        
        headings.forEach(function(heading) {
            if (heading.id && heading.textContent !== 'Table of Contents') {
                const listItem = document.createElement('li');
                const link = document.createElement('a');
                
                link.href = '#' + heading.id;
                link.textContent = heading.textContent.replace('#', '');
                
                if (heading.tagName === 'H3') {
                    listItem.style.marginLeft = '1.5em';
                }
                
                listItem.appendChild(link);
                tocList.appendChild(listItem);
            }
        });
        
        const firstHeading = document.querySelector('h1');
        if (firstHeading && firstHeading.nextElementSibling) {
            content.insertBefore(toc, firstHeading.nextElementSibling);
        } else if (content.firstChild) {
            content.insertBefore(toc, content.firstChild.nextSibling);
        }
        
        toc.style.background = '#f8f9fa';
        toc.style.padding = '1em';
        toc.style.borderRadius = '5px';
        toc.style.marginBottom = '2em';
        toc.style.border = '1px solid #e0e0e0';
    }
}
