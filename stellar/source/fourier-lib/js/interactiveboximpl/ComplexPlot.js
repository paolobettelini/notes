class ComplexPlot extends InteractiveBox {

    /**
     * The color of the axis.
     */
    axisColor = '#FF0000';

    /**
     * The color of the plot.
     */
    plotColor = '#FFFF00';

    /**
     * The color of the magnitude.
     */
    magnitudeColor = '#FFFFFF';

    /**
     * The line width of the magnitude.
     */
    magnitudeWidth = 0.5;

    /**
     * The color of the mini plot.
     */
    miniPlotColor = 'blue';

    /**
     * The horizontal offset of the mini plot.
     */
    miniPlotXOffset = 10;

    /**
     * The horizontal stretch of the mini plot.
     */
    miniPlotXStretch = 2;

    /**
     * The vertical stretch of the mini plot.
     */
    miniPlotYStretch = 0.333;

    /**
     * The vertical offset of the mini plot.
     */
    miniPlotYOffset = this.height * 0.25;

    /**
     * The line width of the axis.
     */
    axisWidth = 1.0;

    /**
     * The line width of the plot.
     */
    plotWidth = 2.0;
    
    /**
     * The current frequency.
     */
    freq = 1;

    #signal = [];
    #counter = 0;

    constructor(name, container, height, width) {
        super(name, container, height, width);
        
        this.setPoints(this.#getSineWave());

        this.#loadAdditionalContent();
    }

    draw(ctx) {
        this.setTime(this.#counter++ / (this.#signal.length - 1));
        if (this.#counter > this.#signal.length) {
            this.#counter = 0;
        }

        this.clearCanvas();
        
        ctx.beginPath();
        ctx.lineWidth = this.axisWidth;
        ctx.strokeStyle = this.axisColor;
        
        ctx.moveTo(this.width / 2, 0);
        ctx.lineTo(this.width / 2, this.height);
        ctx.moveTo(0, this.height / 2);
        ctx.lineTo(this.width, this.height / 2);
        ctx.stroke();
        
        ctx.lineWidth = this.plotWidth;
        ctx.strokeStyle = this.plotColor;
        ctx.beginPath();
        ctx.moveTo(this.width / 2 + this.#signal[0],this.height / 2);
        for (var i = 1; i < this.#counter; i++) {
            let angle = 2 * Math.PI / this.#signal.length * i * this.freq;
            ctx.lineTo(this.width / 2 + this.#signal[i] * Math.cos(angle), this.height / 2 + this.#signal[i] * Math.sin(angle));
        }
        ctx.stroke();

        ctx.strokeStyle = this.miniPlotColor;
        ctx.beginPath();
        ctx.moveTo(this.miniPlotXOffset, this.miniPlotYOffset + this.#signal[0] / 3);
        for (var i = 1; i < this.#counter; i++) {
            ctx.lineTo(this.miniPlotXOffset + this.miniPlotXStretch * i, this.miniPlotYOffset + this.#signal[i] *this.miniPlotYStretch);
        }
        ctx.stroke();

        ctx.strokeStyle = this.magnitudeColor;
        ctx.lineWidth = this.magnitudeWidth;
        ctx.beginPath();
        ctx.moveTo(this.width / 2, this.height / 2);
        let angle = 2 * Math.PI / this.#signal.length * (this.#counter - 1) * this.freq;
        ctx.lineTo(this.width / 2 + this.#signal[this.#counter - 1] * Math.cos(angle), this.height / 2 + this.#signal[this.#counter - 1] * Math.sin(angle));
        ctx.stroke();
    };

    onTimeTravel(value) {
        this.#counter = value * this.#signal.length | 0;
    }

    setPoints(points) {
        this.#signal = [];

        var minY = 10E5;
        var maxY = -10E5;
        for (var i = 0; i < points.length; i++) {
            if (points[i].y < minY) {
                minY = points[i].y;
            }

            if (points[i].y > maxY) {
                maxY = points[i].y;
            }
        }

        //let offset = (minY - maxY) * 0.5;

        for (var i = 0; i < points.length; i++) {
            // Add offset to center signal at x=0
            this.#signal[i] = points[i].y - minY /* + offset */;
        }

        this.#counter = 0;
    };

    #loadAdditionalContent() {
        // todo: refactor
        this.container.insertAdjacentHTML(
            'beforeend', '<br><p class="box" id=' + this.name + '_frequencyLabel>Frequency: 1</p><input class="box" id="' + this.name + '_frequency" type="range" min="0.5" value="1" step="0.05" max="3"><br><button class="box" id="' + this.name + '_sinewave">sin</button><button class="box" id="' + this.name + '_cosinewave">cos</button>');

        // Frequency input
        let freqInput = document.getElementById(this.name + '_frequency');
        let frequencyLabel = document.getElementById(this.name + '_frequencyLabel');
        freqInput.oninput = () => {
            this.freq = freqInput.value;
            this.#counter = 0; // restart
            frequencyLabel.innerHTML = 'Frequency: ' + this.freq;
        }

        // Input Buttons
        document.getElementById(this.name + '_sinewave').onclick = () => this.setPoints(this.#getSineWave());
        document.getElementById(this.name + '_cosinewave').onclick = () => this.setPoints(this.#getCosineWave());
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

    #getCosineWave() {
        var path = [];
        for (var i = 0; i < 100; i++) {
            path[i] = {
                x: 0,
                y: 100 * Math.cos(Math.PI * 2 / 100 * i)
            }
        }
        return path;
    }

}