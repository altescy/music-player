/**
 * Load player
 */

let player = null;
let queue = null;
let controls = null;

let is_ready = false;
let is_setup = false;

window.player_controls = {
    setup: () => {
        console.log("setup ...");
        if(is_ready && !is_setup) {
            player_setup();
            is_setup = true;
        }
        is_ready = true;
        console.log("setup done!");
        return true;
    },
    load: (origin, content_id) => {
        player.load(origin, content_id);
    }
};

function player_setup() {
        player = new Player();
        queue = new Queue(player, '.queue ul');
        controls = new Controls();
        player.onFinish(() => { queue.next(); })
        Sortable.create(document.querySelector('.queue ul'), {
            group: 'track-list',
            animation: 100
        });
        let blind = document.querySelector('.loading-blind');
        if(blind) blind.parentNode.removeChild(blind);
}

function onYouTubeIframeAPIReady() {
    if(is_ready && !is_setup) {
        player_setup();
        is_setup = true;
    }
    is_ready = true;
}



/**
 * Player and Queue controls
 */

let CONTROLS = {
    '.player-toggle': {
        onclick: elem => {
            player.toggle();
        },
        watch: elem => {
            player.isPaused(isPaused => {
                if (isPaused) elem.classList.add('player-is-paused');
                else elem.classList.remove('player-is-paused');
            });
        },
    },
    'input[type=range].player-seekbar': {
        setup: elem => {
            elem.min = '0';
            elem.max = '100';
            elem.value = '0';
            elem.state = '';
            elem.addEventListener('input', () => {
                elem.state = 'input';
            });
            elem.addEventListener('change', () => {
                player.seekTo(Number(elem.value));
                elem.state = '';
            })
        },
        watch: elem => {
            if (elem.state != 'input') {
                player.getPosition(position => {
                    elem.value = position;
                });
            }
            player.getDuration(duration => {
                elem.max = duration;
            });
        },
    },
    '.player-timepassed': {
        watch: elem => {
            player.getPosition(position => {
                if(!position) position = 0;
                elem.textContent = msString(position);
            });
        },
    },
    '.player-duration': {
        watch: elem => {
            player.getDuration(duration => {
                if(!duration) duration = 0;
                elem.textContent = msString(duration);
            });
        },
    },
    '.queue-prev': {
        onclick: elem => {
            queue.prev();
        }
    },
    '.queue-next': {
        onclick: elem => {
            queue.next();
        }
    },
    '.queue-toggle-loop': {
        setup: elem => {
            if (queue.loop) elem.classList.add('queue-loop');
            else elem.classList.remove('queue-loop');
        },
        onclick: elem => {
            queue.toggleLoop();
            if (queue.loop) elem.classList.add('queue-loop');
            else elem.classList.remove('queue-loop');
        }
    }
}

let Controls = function (query) {
    if (!query) query = '.player-controls'
    this.query = query;
    this.controls = {};

    // get elemens and setup controls
    for (let c in CONTROLS) {
        let control = CONTROLS[c];
        let elem = document.querySelector(this.query + ' ' + c);
        if (elem !== null) {
            control.elem = elem;
            if (control.hasOwnProperty('setup')) control.setup(elem);
            this.controls[c] = control;
        }
    }

    // setup `onclick` property
    for (let c in this.controls) {
        let control = this.controls[c];
        if (control.hasOwnProperty('onclick')) {
            control.elem.onclick = () => { control.onclick(control.elem); };
        }
    }

    // setup `watch`
    this.watches = {};
    for (let c in this.controls) {
        let control = this.controls[c];
        if (control.hasOwnProperty('watch')) {
            this.watches[c] = {
                exec: () => { control.watch(control.elem); },
            };
        }
    }
    this.intervalId = setInterval(() => {
        for (let c in this.watches) {
            this.watches[c].exec();
        }
    }, 200);
}


/**
 * Utilities
 */

function zeropad(num) {
    return ("00000000" + num).slice(-2);
}

function msString(milliseconds) {
    sec = milliseconds / 1000;
    let h = Math.floor(sec / 3600);
    let m = Math.floor(sec % 3600 / 60);
    let s = zeropad(Math.floor(sec % 60));
    return (h == 0 ? [m, s] : [h, zeropad(m), s]).join(':');
}
