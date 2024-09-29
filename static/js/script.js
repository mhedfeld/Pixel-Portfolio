document.addEventListener('DOMContentLoaded', function() {
    const parallaxBg = document.querySelector('.parallax-bg');
    const contentWrapper = document.querySelector('.content-wrapper');
    let ticking = false;
    let lastScrollY = 0;
    const parallaxSpeed = 0.5;

    function updateParallax() {
        const scrollY = window.pageYOffset;
        const translateY = scrollY * parallaxSpeed;
        
        parallaxBg.style.transform = `translateY(${-translateY}px)`;
        
        if (scrollY > window.innerHeight) {
            contentWrapper.style.transform = `translateY(${window.innerHeight - scrollY}px)`;
        } else {
            contentWrapper.style.transform = 'translateY(0)';
        }
        
        lastScrollY = scrollY;
        ticking = false;
    }

    window.addEventListener('scroll', function() {
        if (!ticking) {
            window.requestAnimationFrame(updateParallax);
            ticking = true;
        }
    });
});