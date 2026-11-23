/* MEOCOC — site.js · vanilla, dùng chung mọi trang */
(function(){
  var qs = location.search;
  var staticMode = qs.indexOf('static') > -1;
  var reduced = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
  document.documentElement.classList.add('js');
  if(staticMode) document.documentElement.classList.add('no-anim');

  /* helper chụp ảnh: ?static=1&scroll=<section-id> */
  var sm = location.search.match(/[?&]scroll=([a-z-]+)/i);
  if(sm){
    window.addEventListener('load', function(){
      var el = document.getElementById(sm[1]);
      if(el){
        el.scrollIntoView({behavior:'instant', block:'start'});
        window.scrollBy({top:-70, behavior:'instant'});
      }
    });
  }

  /* reveal on scroll + đếm số */
  var revealEls = [].slice.call(document.querySelectorAll('.rv'));
  var counters = [].slice.call(document.querySelectorAll('[data-count]'));
  function runCounter(el){
    var target = parseInt(el.getAttribute('data-count'),10), t0 = null, dur = 1200;
    function step(t){
      if(!t0) t0 = t;
      var p = Math.min((t - t0)/dur, 1);
      p = 1 - Math.pow(1-p, 3);
      el.textContent = Math.round(target * p);
      if(p < 1) requestAnimationFrame(step);
    }
    requestAnimationFrame(step);
  }
  function startObservers(){
    if('IntersectionObserver' in window && !staticMode){
      var io = new IntersectionObserver(function(entries){
        entries.forEach(function(en){
          if(en.isIntersecting){ en.target.classList.add('on'); io.unobserve(en.target); }
        });
      }, {threshold:.1, rootMargin:'0px 0px -6% 0px'});
      revealEls.forEach(function(el){ io.observe(el); });
    } else {
      revealEls.forEach(function(el){ el.classList.add('on'); });
    }
    if('IntersectionObserver' in window && !staticMode && !reduced){
      var cio = new IntersectionObserver(function(entries){
        entries.forEach(function(en){
          if(en.isIntersecting){ runCounter(en.target); cio.unobserve(en.target); }
        });
      }, {threshold:.4});
      counters.forEach(function(el){ cio.observe(el); });
    } else {
      counters.forEach(function(el){ el.textContent = el.getAttribute('data-count'); });
    }
  }
  startObservers();

  /* header: nền mờ khi scroll + read progress (trang bài) — một vòng rAF */
  var hdr = document.getElementById('hdr');
  var readBar = document.getElementById('readbar');
  var lastY = 0, ticking = false;
  function onScrollFrame(){
    var y = window.scrollY;
    if(readBar){
      var dh = document.documentElement.scrollHeight - window.innerHeight;
      readBar.style.transform = 'scaleX(' + (dh > 0 ? Math.min(y/dh, 1) : 0) + ')';
    }
    if(hdr) hdr.classList.toggle('scrolled', y > 12);
    lastY = y;
    ticking = false;
  }
  window.addEventListener('scroll', function(){
    if(!ticking){ requestAnimationFrame(onScrollFrame); ticking = true; }
  }, {passive:true});
  onScrollFrame();

  /* nav link active theo section (trang chủ) */
  var sections = [].slice.call(document.querySelectorAll('main section[id]'));
  var navLinks = [].slice.call(document.querySelectorAll('.nav-links a, .nav-panel a'));
  if('IntersectionObserver' in window && sections.length){
    var sio = new IntersectionObserver(function(entries){
      entries.forEach(function(en){
        if(en.isIntersecting){
          navLinks.forEach(function(a){
            a.classList.toggle('act', a.getAttribute('href') === '#' + en.target.id ||
              a.getAttribute('href') === '/#' + en.target.id ||
              a.getAttribute('href') === '/en/#' + en.target.id);
          });
        }
      });
    }, {rootMargin:'-40% 0px -55% 0px'});
    sections.forEach(function(s){ sio.observe(s); });
  }

  /* copy email */
  var copyBtn = document.getElementById('copyBtn');
  var copied = document.getElementById('copied');
  if(copyBtn){
    copyBtn.addEventListener('click', function(){
      var done = function(){ copied.classList.add('show'); setTimeout(function(){ copied.classList.remove('show'); }, 1800); };
      function fallback(){
        var ta = document.createElement('textarea');
        ta.value = 'toilatruc999@gmail.com';
        ta.style.position = 'fixed'; ta.style.opacity = '0';
        document.body.appendChild(ta); ta.select();
        try{ document.execCommand('copy'); }catch(e){}
        document.body.removeChild(ta); done();
      }
      if(navigator.clipboard && navigator.clipboard.writeText){
        navigator.clipboard.writeText('toilatruc999@gmail.com').then(done, fallback);
      } else {
        fallback();
      }
    });
  }

  /* menu mobile */
  var menuBtn = document.getElementById('menuBtn');
  var navPanel = document.getElementById('navPanel');
  if(menuBtn && navPanel){
    var isEn = document.documentElement.lang === 'en';
    var setMenu = function(open){
      navPanel.classList.toggle('open', open);
      menuBtn.classList.toggle('open', open);
      menuBtn.setAttribute('aria-expanded', open ? 'true' : 'false');
      menuBtn.setAttribute('aria-label', open ? (isEn ? 'Close menu' : 'Đóng menu') : (isEn ? 'Open menu' : 'Mở menu'));
    };
    menuBtn.addEventListener('click', function(){ setMenu(!navPanel.classList.contains('open')); });
    [].slice.call(navPanel.querySelectorAll('a')).forEach(function(a){
      a.addEventListener('click', function(){ setMenu(false); });
    });
    window.addEventListener('resize', function(){ if(window.innerWidth > 960) setMenu(false); });
  }
})();
