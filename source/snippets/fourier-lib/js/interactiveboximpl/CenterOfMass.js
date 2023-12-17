class CenterOfMass extends InteractiveBox {

    /**
     * The color of the Cartesian axis.
     */
    axisColor = '#FF0000';

    /**
     * The color of the plot.
     */
    plotColor = '#FFFF00';

    /**
     * The color of the center of mass.
     */
    centerOfMassColor = "#3385ff";

    /**
     * The line width of the axis.
     */
    axisWidth = 1.0;

    /**
     * The line width of the plot.
     */
    plotWidth = 2.0;

    /**
     * The center of mass dot size.
     */
    centerOfMassWidth = 10;
    
    /**
     * The full rotations computed.
     */
    rotations = 3;

    #signal = [];

    /**
     * The current frequency.
     */
    #freq = 1;

    /**
     * The frequency step.
     */
    #freqStep = 0.01;

    /**
     * The minimum frequency.
     */
    #minFreq = 0.25;

    /**
     * The maximum frequency.
     */
    #maxFreq = 5;

    constructor(name, container, height, width) {
        super(name, container, height, width);
        
        this.setPoints(this.#getSineWave());

        this.#loadAdditionalContent();
    }

    gay = 0;

    draw(ctx) {
        if (this.#signal.length < 2)
            return;

        this.setTime(((this.#freq += this.#freqStep) - this.#minFreq) / (this.#maxFreq - this.#freqStep - this.#minFreq));
        if (this.#freq > this.#maxFreq) {
            this.#freq = this.#minFreq;
        }

        this.clearCanvas();

        // constants
        const HALF_WIDTH = this.width / 2;
        const HALF_HEIGHT = this.height / 2;
        const HALF_PI = Math.PI / 2
        
        ctx.beginPath();
        ctx.lineWidth = this.axisWidth;
        ctx.strokeStyle = this.axisColor;
        
        ctx.moveTo(HALF_WIDTH, 0);
        ctx.lineTo(HALF_WIDTH, this.height);
        ctx.moveTo(0, HALF_HEIGHT);
        ctx.lineTo(this.width, HALF_HEIGHT);
        ctx.stroke();

        ctx.lineWidth = this.plotWidth;
        ctx.strokeStyle = this.plotColor;
        ctx.beginPath();

        let centerOfMassX = 0;
        let centerOfMassY = 0;

        // Double rotation

        let a = 2 * Math.PI / this.#signal.length * this.#freq;
        ctx.moveTo(HALF_WIDTH, HALF_HEIGHT + this.#signal[0]);
        for (var j = 0; j < this.rotations; j++) {
            for (var i = 1; i < this.#signal.length; i++) {
                let angle = a * (i + this.#signal.length * j) + HALF_PI;
                let x = HALF_WIDTH + this.#signal[i] * Math.cos(angle);
                let y = HALF_HEIGHT + this.#signal[i] * Math.sin(angle);
                ctx.lineTo(x, y);
                centerOfMassX += x;
                centerOfMassY += y;
            }
        }
        ctx.stroke();

        centerOfMassX /= this.#signal.length * this.rotations;
        centerOfMassY /= this.#signal.length * this.rotations;

        
        ctx.strokeStyle = this.centerOfMassColor;
        ctx.lineWidth = this.centerOfMassWidth;
        ctx.beginPath();
        ctx.arc(centerOfMassX, centerOfMassY, 5, 0, Math.PI * 2);
        ctx.stroke();
        ctx.lineWidth = this.axisWidth;
    };

    onTimeTravel(value) {
        this.#freq = this.#minFreq + value * (this.#maxFreq - this.#minFreq);
    }

    setPoints(points) {
        this.#signal = [];

        var minY = 1E5;
        var maxY = -1E5;
        for (var i = 0; i < points.length; i++) {
            if (points[i].y < minY) {
                minY = points[i].y;
            }

            if (points[i].y > maxY) {
                maxY = points[i].y;
            }
        }

        let offset = (minY - maxY) * 0.5;

        for (var i = 0; i < points.length; i++) {
            this.#signal[i] = points[i].y - minY + offset;
        }

        this.#freq = this.#minFreq;
    };

    #loadAdditionalContent() {
        this.container.insertAdjacentHTML(
            'beforeend', '<br><button class="box" id="' + this.name + '_sinewave">sine</button>');

        // Sine Wave Button
        document.getElementById(this.name + '_sinewave').onclick = () => this.setPoints(this.#getSineWave());
    }

    #getSineWave() {
        var path = [];
        for (var i = 0; i < 100; i++) {
            path[i] = {
                x: 0,
                y: 100 * Math.sin(Math.PI * 2 / 100 * i)
            }
        }

        return path;
    }


}