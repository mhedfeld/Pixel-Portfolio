document.addEventListener('DOMContentLoaded', function() {
    const parallaxBg = document.querySelector('.parallax-bg');
    const contentWrapper = document.querySelector('.content-wrapper');
    console.log("JavaScript is loaded and running");
    const projectContainer = document.querySelector('.project-container');
    if (!projectContainer) {
        console.error("Project container not found");
        return;
    }
    let ticking = false;
    const parallaxSpeed = 0.5;

    function updateParallax() {
        const scrollY = window.pageYOffset;
        const translateY = scrollY * parallaxSpeed;
        parallaxBg.style.transform = `translateY(${-translateY}px)`;
        if (scrollY > window.innerHeight) {
            contentWrapper.style.transform = `translateY(${Math.min(window.innerHeight - scrollY, 0)}px)`;
        } else {
            contentWrapper.style.transform = 'translateY(0)';
        }
        ticking = false;
    }
    window.addEventListener('scroll', function() {
        if (!ticking) {
            window.requestAnimationFrame(updateParallax);
            ticking = true;
        }
    });

// Fetch and display projects
if (projectContainer) {
    fetch('/projects_data')
        .then(response => response.json())
        .then(data => {
            console.log("Data Fetched:", data);
            data.forEach(project => {
                console.log("Creating card for project:", project);
                const projectCard = document.createElement('div');
                projectCard.className = 'project-card';
                projectCard.innerHTML = `
                    <img src="${project.image}" alt="${project.title} Image" class="project-image">
                    <h2>${project.title}</h2>
                    <p>${project.description.substring(0, 100)}...</p>
                    <a href="${project.link}" class="learn-more-link">Learn more</a>
                `;
                projectContainer.appendChild(projectCard);

                const learnMoreLink = projectCard.querySelector('.learn-more-link');
                console.log("Learn more link:", learnMoreLink);
                learnMoreLink.addEventListener('click', function(event) {
                    event.preventDefault();
                    console.log(`Click event fired for project ${project.id}`);
                    console.log(`Navigating to: ${this.href}`);
                    window.location.href = this.href;
                });
            });
        })
        .catch(error => console.error('Error fetching projects:', error));
}
});
