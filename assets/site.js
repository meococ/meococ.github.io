/* MEOCOC — site.js · dùng chung mọi trang (vanilla, không thư viện) */
(function(){
  var qs = location.search;
  var staticMode = qs.indexOf('static') > -1;
  var reduced = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
  document.documentElement.classList.add('js');
  if(staticMode) document.documentElement.classList.add('no-anim');

  /* hỗ trợ chụp ảnh preview: ?static=1&scroll=xxx */
  var sm = location.search.match(/[?&]scroll=([a-z]+)/i);
  if(sm){
    window.addEventListener('load', function(){
      var el = document.getElementById(sm[1]);
      if(el){
        el.scrollIntoView({behavior:'instant', block:'start'});
        window.scrollBy({top:-70, behavior:'instant'});
      }
    });
  }
  var om = location.search.match(/[?&]only=([a-z]+)/i);
  if(om){
    var onlyId = om[1];
    [].slice.call(document.querySelectorAll('main > section, main .mq-wrap')).forEach(function(s){
      if(s.id !== onlyId && !s.classList.contains(onlyId)) s.style.display = 'none';
    });
    var hdr0 = document.getElementById('hdr');
    if(hdr0) hdr0.style.position = 'absolute';
    var fl = document.querySelector('.rail-l'); if(fl) fl.style.display = 'none';
  }

  /* reveal + đếm số */
  var revealEls = [].slice.call(document.querySelectorAll('.rv'));
  var scribbles = [].slice.call(document.querySelectorAll('.scribble'));
  var counters = [].slice.call(document.querySelectorAll('[data-count]'));
  function runCounter(el){
    var target = parseInt(el.getAttribute('data-count'),10), t0 = null, dur = 1300;
    function step(t){
      if(!t0) t0 = t;
      var p = Math.min((t - t0)/dur, 1);
      p = 1 - Math.pow(1-p, 3);
      el.textContent = Math.round(target * p);
      if(p < 1) requestAnimationFrame(step);
    }
    requestAnimationFrame(step);
  }

  /* intro — chỉ trang chủ có #intro */
  var intro = document.getElementById('intro');
  var seen = false;
  try{ seen = sessionStorage.getItem('mc-intro') === '1'; }catch(e){}
  function startObservers(){
    if('IntersectionObserver' in window && !staticMode){
      var io = new IntersectionObserver(function(entries){
        entries.forEach(function(en){
          if(en.isIntersecting){
            en.target.classList.add('on');
            io.unobserve(en.target);
          }
        });
      }, {threshold:.12, rootMargin:'0px 0px -8% 0px'});
      revealEls.forEach(function(el){ io.observe(el); });
      scribbles.forEach(function(el){ io.observe(el); });
    } else {
      revealEls.forEach(function(el){ el.classList.add('on'); });
      scribbles.forEach(function(el){ el.classList.add('on'); });
    }
    if('IntersectionObserver' in window && !staticMode && !reduced){
      var cio = new IntersectionObserver(function(entries){
        entries.forEach(function(en){
          if(en.isIntersecting){ runCounter(en.target); cio.unobserve(en.target); }
        });
      }, {threshold:.5});
      counters.forEach(function(el){ cio.observe(el); });
    } else {
      counters.forEach(function(el){ el.textContent = el.getAttribute('data-count'); });
    }
  }
  if(staticMode || reduced || seen || !intro){
    if(intro) intro.remove();
    document.body.classList.add('ready');
    startObservers();
  } else {
    document.documentElement.style.overflow = 'hidden';
    setTimeout(function(){
      intro.classList.add('leave');
      document.documentElement.style.overflow = '';
      try{ sessionStorage.setItem('mc-intro','1'); }catch(e){}
      setTimeout(function(){ if(intro.parentNode) intro.remove(); }, 1000);
    }, 1250);
    /* hero lắp ráp khi màn đen đang nhấc lên */
    setTimeout(function(){
      document.body.classList.add('ready');
      startObservers();
    }, 1450);
  }

  /* nav ẩn/hiện + progress + thanh đọc bài — một vòng rAF duy nhất */
  var hdr = document.getElementById('hdr');
  var bar = document.getElementById('progress');
  var readBar = document.getElementById('readbar');
  var lastY = 0, ticking = false;
  function onScrollFrame(){
    var y = window.scrollY;
    if(bar){
      var max = document.documentElement.scrollHeight - window.innerHeight;
      bar.style.transform = 'scaleY(' + (max > 0 ? y/max : 0) + ')';
    }
    if(readBar){
      var dh = document.documentElement.scrollHeight - window.innerHeight;
      readBar.style.transform = 'scaleX(' + (dh > 0 ? Math.min(y/dh, 1) : 0) + ')';
    }
    if(hdr){
      hdr.classList.toggle('scrolled', y > 24);
      hdr.classList.toggle('hide', y > lastY && y > 160);
    }
    lastY = y;
    ticking = false;
  }
  window.addEventListener('scroll', function(){
    if(!ticking){ requestAnimationFrame(onScrollFrame); ticking = true; }
  }, {passive:true});

  /* nav link active theo section (trang chủ) */
  var sections = [].slice.call(document.querySelectorAll('main > section[id]'));
  var navLinks = [].slice.call(document.querySelectorAll('.nav-links a'));
  if('IntersectionObserver' in window && sections.length){
    var sio = new IntersectionObserver(function(entries){
      entries.forEach(function(en){
        if(en.isIntersecting){
          navLinks.forEach(function(a){
            a.classList.toggle('act', a.getAttribute('href') === '#' + en.target.id ||
              a.getAttribute('href') === '/#' + en.target.id);
          });
        }
      });
    }, {rootMargin:'-45% 0px -50% 0px'});
    sections.forEach(function(s){ sio.observe(s); });
  }

  /* copy email (trang chủ) */
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
})();
