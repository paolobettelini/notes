class InteractiveBox {

    static PAUSED = '&#9658;';
    static PLAYING = '&#10074;&#10074;';

    static #names = [];

    name;
    height;
    width;
    container;
    
    #schedulerTask = -1;
    #canvas;
    #ctx;
    #timeline;
    #playButton;

    #wasPlaying;

    /**
     * 
     * @param {String} name the name of the interactive box
     * @param {String} container the id of the container element
     * @param {Number} height the height of the canvas
     * @param {Number} width the width of the canvas
     */
    constructor(name, container, height, width) {
        // check for name conflict
        if (InteractiveBox.#names.includes(name)) {
            throw 'The name \'' + name + '\' is already being used! Names should be unique to avoid conflicts.'
        }

        // Add name to static list
        InteractiveBox.#names.push(name);

        this.name = name;
        this.height = height;
        this.width = width;
        this.#wasPlaying = true;
        this.container = document.getElementById(container);

        this.container.innerHTML = ''
            + '<canvas class="box" id="' + name + '_canvas" width="' + width + '" height="' + height + '"></canvas>'
            + '<br>'
            + '<div class="box" style="width:' + width + 'px">'
            + '<button class="box" id="' + name + '_playButton">&#10074;&#10074;</button>'
            + '<input class="box" id="' + name + '_timeline" type="range" min="0" max="1" step="0.01"></input>'
            + '<div>';

        this.#timeline = document.getElementById(name + '_timeline')
        this.#canvas = document.getElementById(name + '_canvas');
        this.#ctx = this.#canvas.getContext('2d');
        this.#playButton = document.getElementById(name + '_playButton');

        this.#playButton.onclick = () => {
            this.#playButtonClick();
        };

        this.#initTimeline();
        this.#initDrawablePath();

        // Pause/Resume based on visibility
        document.addEventListener('scroll', () => this.#onScroll());

        // Start if visible
        this.#onScroll();
    }

    /***
     * Simulates pressing the play/stop button.
     */
    #playButtonClick() {
        this.#playButton.innerHTML = this.toggle() ? InteractiveBox.PLAYING : InteractiveBox.PAUSED
    }

    /**
     * Updates the timeline.
     * 
     * @param {Number} value the time value btween 0 and 1
     */
    setTime(value) {
        this.#timeline.value = value;
    }

    /**
     * Resumes the animation
     */
    resume() {
        if (this.#isElementInViewport(this.#canvas)) {
            this.#schedulerTask = setInterval(() => this.draw(this.#ctx), 25);
            this.#playButton.innerHTML = InteractiveBox.PLAYING;
        }
    }

    /**
     * Pauses the animation
     */
    pause() {
        clearInterval(this.#schedulerTask);
        this.#schedulerTask = -1;
        this.#playButton.innerHTML = InteractiveBox.PAUSED;
    }

    /**
     * CHecks if the animation is playing.
     * 
     * @returns {boolean} if the animation is playing
     */
    isPlaying() {
        return this.#schedulerTask != -1;
    }
    
    /**
     * Stops the animation if it is playing, resumes the animation
     * otherwise.
     * 
     * @returns {Boolean} the state of the animation
     */
    toggle() {
        this.isPlaying() ? this.pause() : this.resume();
        return this.isPlaying();
    }

    /**
     * Renders the current frame on the canvas.
     * This function should be overwritten.
     * 
     * @param {CanvasRenderingContext2D} ctx 
     */
    draw(ctx) {
        throw 'The function draw() has not been overwritten'
    }

    /**
     * Updates the input of the anumation.
     * This function should be overwritten.
     * 
     * @param {{x: Number, y: Number}[]} points the points
     */
    setPoints(points) {
        throw 'The function setPoints(points) has not been overwritten'
    }

    /**
     * Called when the user interacts with the timeline.
     * This function should be overwritten.
     * 
     * @param {Number} value the time value between 0 and 1
     */
    onTimeTravel(value) {
        throw 'The function onTimeTravel(value) has not been overwritten'
    }

    /**
     * Clears the canvas
     */
    clearCanvas() {
        this.#ctx.clearRect(0, 0, this.#canvas.width, this.#canvas.height);
    }

    // drawable path
    #drawingPath = false;
    #points = [];

    /**
     * Initializes the drawable path on the canvas.
     */
    #initDrawablePath() {
        this.#canvas.onmousedown = () => {
            this.#drawingPath = true;
            this.pause();
            this.clearCanvas();
            
            // show path being drawn
            this.#ctx.beginPath();
        };

        this.#canvas.onmouseup = () => {
            this.#drawingPath = false;
            
            while (this.#points.length > 200) {
                //remove random points
                this.#points.splice((Math.random() * this.#points.length) | 0, 1);
            }

            if (this.#points.length > 2) {
                this.setPoints(this.#points);
                this.resume();
            } else {
                // toggle state (?)
            }

            this.#points = [];
        };

        this.#canvas.onmousemove = e => {
            if (!this.#drawingPath) {
                return;
            }

            // show path being drawn
            if (this.#points.length == 0) {
                this.#ctx.moveTo(e.offsetX, e.offsetY);
            } else {
                this.#ctx.lineTo(e.offsetX, e.offsetY);
                this.#ctx.stroke();
            }

            this.#points.push({
                x: e.offsetX,
                y: e.offsetY
            });
        };
    }

    #wasTimelinePlaying = false;

    /**
     * Initializes the interactible timeline.
     */
    #initTimeline() {
        this.#timeline.onmousedown = _ => {
            this.#wasTimelinePlaying = this.isPlaying();
            this.pause();
        };

        this.#timeline.onmouseup = _ => {
            if (this.#wasTimelinePlaying) {
                this.resume();
            }
        };

        this.#timeline.oninput = () => {
            this.onTimeTravel(this.#timeline.value);
            this.setTime(this.#timeline.value);
            this.draw(this.#ctx);
        }
    }

    /**
     * Checks if the elements is at least a bit visible in the viewport.
     * 
     * @param {HTMLElement} element the element to check
     * @returns {Boolean}
     */
    #isElementInViewport(element) {
        var rect = element.getBoundingClientRect();
        return (
            rect.top >= 0 &&
            rect.top <= (window.innerHeight || document.documentElement.clientHeight) ||
            rect.bottom >= 0 &&
            rect.bottom <= (window.innerHeight || document.documentElement.clientHeight)
        );
    }

    /**
     * Pause/Resume rendering on scroll if the canvas is not visible
     * The previous status of the box is preserved.
     */
    #onScroll() {
        if (this.#isElementInViewport(this.#canvas)) {
            if (!this.isPlaying() && this.#wasPlaying) {
                this.resume();
                this.#wasPlaying = false;
            }
        } else {
            if (this.isPlaying()) {
                this.#wasPlaying = true;
                this.pause();
            }
        }
    }

}