// let player = null;
// let queue = null;
// let controls = null;


/* --- Player definition --- */

let AbstractPlayer = function (query) { this.api = undefined; };
AbstractPlayer.prototype.load = function (content_id) { throw new Error('Not Implemented'); };
AbstractPlayer.prototype.play = function () { throw new Error('Not Implemented'); };
AbstractPlayer.prototype.pause = function () { throw new Error('Not Implemented'); };
AbstractPlayer.prototype.show = function () { throw new Error('Not Implemented'); };
AbstractPlayer.prototype.hide = function () { throw new Error('Not Implemented'); };
AbstractPlayer.prototype.stop = function () { throw new Error('Not Implemented'); };
AbstractPlayer.prototype.seekTo = function (milliseconds) { throw new Error('Not Implemented'); };
AbstractPlayer.prototype.setVolume = function (volume) { throw new Error('Not Implemented'); };
AbstractPlayer.prototype.getVolume = function (callback) { throw new Error('Not Implemented'); };
AbstractPlayer.prototype.getDuration = function (callback) { throw new Error('Not Implemented'); };
AbstractPlayer.prototype.getPosition = function (callback) { throw new Error('Not Implemented'); };
AbstractPlayer.prototype.isPaused = function (callback) { throw new Error('Not Implemented'); };
AbstractPlayer.prototype.getElement = function () { throw new Error('Not Implemented'); };
AbstractPlayer.prototype.onFinish = function (callback) { throw new Error('Not Implemented'); };


function inherits(childCtor, parentCtor) {
    Object.setPrototypeOf(childCtor.prototype, parentCtor.prototype);
};

function addApiScript(src) {
    let tag = document.createElement('script');
    tag.src = src;
    let firstScriptTag = document.getElementsByTagName('script')[0];
    firstScriptTag.parentNode.insertBefore(tag, firstScriptTag);
}


/**
 * YouTube iframe settings
 *     more info.: https://developers.google.com/youtube/iframe_api_reference
 */

addApiScript('https://www.youtube.com/iframe_api');

let YTPlayer = function (parent) {
    if (!parent) parent = '#player';
    this.query = '#yt-player';
    if (document.querySelector(this.query) === null) {
        let yt_player = document.createElement('div');
        yt_player.id = this.query.slice(1);
        document.querySelector(parent).appendChild(yt_player);
    }
    this.api = new YT.Player(this.query.slice(1), {
        playerVars: {
            controls: 0,
            disablekb: 1,
            enablejsapi: 1,
            fs: 1,
            playsinline: 1,
            modestbranding: 1,
            rel: 0,
            showinfo: 0,
            iv_load_policy: 3,
            origin: 'http://localhost'
        },
    });
};
inherits(YTPlayer, AbstractPlayer);
YTPlayer.prototype.load = function (content_id) {
    this.api.loadVideoById({
        'videoId': content_id,
        'startSeconds': 0,
    });
};
YTPlayer.prototype.play = function () { if (!this.isHidden()) this.api.playVideo(); };
YTPlayer.prototype.pause = function () { this.api.pauseVideo(); };
YTPlayer.prototype.show = function () { document.querySelector(this.query).style.display = "block"; };
YTPlayer.prototype.hide = function () { document.querySelector(this.query).style.display = "none"; };
YTPlayer.prototype.stop = function () { this.hide(); this.pause(); };
YTPlayer.prototype.seekTo = function (milliseconds) { this.api.seekTo(milliseconds / 1000.0, true); };
YTPlayer.prototype.setVolume = function (volume) { this.api.setVolume(volume); };
YTPlayer.prototype.getVolume = function (callback) { callback(this.api.getVolume()); };
YTPlayer.prototype.getDuration = function (callback) { callback(1000 * this.api.getDuration()); };
YTPlayer.prototype.getPosition = function (callback) { callback(1000 * this.api.getCurrentTime()); };
YTPlayer.prototype.isPaused = function (callback) { callback(this.api.getPlayerState() !== YT.PlayerState.PLAYING); };
YTPlayer.prototype.isHidden = function () { return document.querySelector(this.query).style.display == "none"; };
YTPlayer.prototype.getElement = function () { return document.querySelector(this.query); };
YTPlayer.prototype.onFinish = function (callback) {
    this.api.addEventListener('onStateChange', event => {
        if (event.target.getPlayerState() === YT.PlayerState.ENDED) callback();
    });
};


/**
 * SoundCloud iframe setting
 *     more info.: https://developers.soundcloud.com/docs/api/html5-widget
 */

// addApiScript('https://w.soundcloud.com/player/api.js');

let SCPlayer = function (parent) {
    if (!parent) parent = '#player';
    this.query = '#sc-player';
    if (document.querySelector(this.query) === null) {
        let sc_player = document.createElement('iframe');
        sc_player.id = this.query.slice(1);
        sc_player.scrolling = "no";
        sc_player.frameborder = "no";
        sc_player.allow = "autoplay; encrypted-media";
        sc_player.src = "https://w.soundcloud.com/player/?url=https%3A//api.soundcloud.com/tracks/293&amp;";
        document.querySelector(parent).appendChild(sc_player);
    }
    this.api = SC.Widget(this.query.slice(1));
};
inherits(SCPlayer, AbstractPlayer);
SCPlayer.prototype.load = function (content_id) {
    let url = 'https%3A//api.soundcloud.com/tracks/' + content_id;
    let options = {
        auto_play: 1,
        download: 0,
        hide_related: 1,
        liking: 0,
        sharing: 0,
        show_artwork: 0,
        show_comments: 0,
        show_user: 0,
        show_reports: 0,
        show_teaser: 0,
        visual: 1
    }
    this.api.load(url, options);
}
SCPlayer.prototype.play = function () { if (!this.isHidden()) this.api.play(); };
SCPlayer.prototype.pause = function () { this.api.pause(); };
SCPlayer.prototype.show = function () { document.querySelector(this.query).style.display = "block"; };
SCPlayer.prototype.hide = function () { document.querySelector(this.query).style.display = "none"; };
SCPlayer.prototype.stop = function () { this.hide(); this.pause(); };
SCPlayer.prototype.seekTo = function (milliseconds) { this.api.seekTo(milliseconds); };
SCPlayer.prototype.setVolume = function (volume) { this.api.setVolume(volume); };
SCPlayer.prototype.getVolume = function (callback) { this.api.getVolume(callback); };
SCPlayer.prototype.getDuration = function (callback) { this.api.getDuration(callback); };
SCPlayer.prototype.getPosition = function (callback) { this.api.getPosition(callback); };
SCPlayer.prototype.isPaused = function (callback) { this.api.isPaused(callback); };
SCPlayer.prototype.isHidden = function () { return document.querySelector(this.query).style.display == "none"; };
SCPlayer.prototype.getElement = function () { return document.querySelector(this.query); };
SCPlayer.prototype.onFinish = function (callback) { this.api.bind(SC.Widget.Events.FINISH, callback); };


/**
 * Player
 */

let Player = function () {
    this.players = {
        youtube: new YTPlayer(),
        soundcloud: new SCPlayer(),
    };
    this.origin = null;
    for (let o in this.players) {
        this.players[o].hide();
    }
}
Player.prototype.load = function (origin, content_id) {
    this.origin = null;
    for (let o in this.players) {
        if (o == origin) {
            this.origin = o;
        } else {
            this.players[o].stop();
        }
    }
    if (!this.origin) throw new Error('invalid origin: ' + origin);

    this.players[this.origin].load(content_id);
    this.players[this.origin].show();
}
Player.prototype.play = function () { if (this.origin) this.players[this.origin].play(); }
Player.prototype.pause = function () { if (this.origin) this.players[this.origin].pause(); }
Player.prototype.stop = function () { if (this.origin) this.players[this.origin].stop(); }
Player.prototype.seekTo = function (milliseconds) { if (this.origin) this.players[this.origin].seekTo(milliseconds); };
Player.prototype.setVolume = function (volume) { if (this.origin) this.players[this.origin].setVolume(volume); };
Player.prototype.getVolume = function (callback) { if (this.origin) this.players[this.origin].getVolume(callback); };
Player.prototype.getDuration = function (callback) { if (this.origin) this.players[this.origin].getDuration(callback); };
Player.prototype.getPosition = function (callback) { if (this.origin) this.players[this.origin].getPosition(callback); };
Player.prototype.isPaused = function (callback) { if (this.origin) this.players[this.origin].isPaused(callback); };
Player.prototype.getElement = function () { if (this.origin) return this.players[this.origin].getElement(); };
Player.prototype.onFinish = function (callback) {
    for (let o in this.players) {
        this.players[o].onFinish(callback);
    }
}
Player.prototype.toggle = function () {
    if (!this.origin) return;
    this.players[this.origin].isPaused(isPaused => {
        if (isPaused) this.players[this.origin].play();
        else this.players[this.origin].pause();
    });
}


/**
 * Queue
 */

let Queue = function (player, query) {
    this.player = player;
    this.query = query? query:'#queue';
    this.loop = false;
}
Queue.prototype.getNowPlaying = function () {
    return document.querySelector(this.query + ' .now-playing');
}
Queue.prototype.setLoop = function () {
    this.loop = true;
}
Queue.prototype.unsetLoop = function () {
    this.loop = false;
}
Queue.prototype.toggleLoop = function () {
    this.loop = !this.loop;
}
Queue.prototype.prev = function () {
    let curr = this.getNowPlaying();
    if (curr) {
        let prev = curr.parentNode.previousElementSibling;
        if (!prev && this.loop) {
            prev = document.querySelector(this.query).lastElementChild;
        }
        if (prev) prev.querySelector('.track-container .track-title').click();
    } else {
        this.player.stop();
    }
}
Queue.prototype.next = function () {
    let curr = this.getNowPlaying();
    if (curr) {
        let next = curr.parentNode.nextElementSibling;
        if (!next && this.loop) {
            next = document.querySelector(this.query).firstElementChild;
        }
        if (next) next.querySelector('.track-container .track-title').click();
    } else {
        this.player.stop();
    }
}

