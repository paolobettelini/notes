class FourierSeries1D extends InteractiveBox {

    /**
     * The color of the epicycles.
     */
    epicyclesColor = '#00ccff';

    /**
     * The colors of the lines.
     */
    linesColor = '#ffffff';

    /**
     * The color of the path.
     */
    pathColor = '#ffff00';

    /**
     * The left offset of the horizontal epicycles.
     */
    epicyclesXOffset = 100;

    /**
     * The top offset of the horizontal epicycles.
     */
    epicyclesYOffset = 250;

    /**
     * The distance of the path.
     */
    pathDistance = 100;

    #counter = 0;
    #signal = [];
    #signal_DFT;

    constructor(name, container, height, width) {
        super(name, container, height, width)

        this.setPoints(this.#getSineWave());

        this.#loadAdditionalContent();
    }

    draw(ctx) {
        // Set time
        this.setTime(this.#counter++ / (this.#signal.length - 1));
        if (this.#counter > this.#signal.length) {
            this.#counter = 0;
        }

        this.clearCanvas();
        
        // Draw epicycles
        var lastPoint = this.drawEpicycles(ctx, this.#signal_DFT, this.epicyclesXOffset, this.epicyclesYOffset, Math.PI * 0.5);
        
        ctx.strokeStyle = this.linesColor;
        ctx.beginPath();
        
        ctx.moveTo(this.epicyclesXOffset + lastPoint.x, this.epicyclesYOffset + lastPoint.y);
        ctx.lineWidth = 0.25;
        ctx.lineTo(this.epicyclesXOffset + lastPoint.x + 100, this.epicyclesYOffset + lastPoint.y);
        
        ctx.stroke();
        ctx.lineWidth = 0.5;
        //ctx.lineTo(this.verticalEpicyclesXOffset + lastPointX.x, this.verticalEpicyclesYOffset + lastPointX.y);
        ctx.stroke();
        
        ctx.beginPath();
        ctx.lineWidth = 2.0;
        ctx.strokeStyle = this.pathColor;
        ctx.moveTo(this.epicyclesXOffset + this.pathDistance + (this.#counter - 1), this.#signal[0] + this.epicyclesYOffset);
        for (var i = 1; i < this.#counter; i++) {
            ctx.lineTo(this.epicyclesXOffset + this.pathDistance + (this.#counter - i - 1), this.#signal[i] + this.epicyclesYOffset);
        }
        
        ctx.stroke();
    };

    onTimeTravel(value) {
        this.#counter = value * this.#signal.length | 0;
    }

    setPoints(points) {
        this.#signal = [];

        // Remove the yOffset, then center the signal at x=0

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

        let offset = (minY - maxY) * 0.5;

        for (var i = 0; i < 2; i++) {
            for (var j = 0; j < points.length; j++) {
                let v = points[points.length - j - 1].y - minY + offset;
                this.#signal.push(v);
                this.#signal.push(v); // double-wide
            }
        }

        this.#counter = 0;

        this.#signal_DFT = Fourier.dft(this.#signal);
    }

    /**
     * Draws a set of epicycles on the canvas.
     * 
     * @param {CanvasRenderingContext2D} ctx the canvas rendering context
     * @param {{Re: Number, Im: Number}[]} fourierTransform 
     * @param {Number} xOff the x offset
     * @param {Number} yOff the y offset
     * @param {Number} rot  the rotation of the epicycles
     * @returns 
     */
    drawEpicycles(ctx, fourierTransform, xOff, yOff, rot) {
        let x = 0;
        let y = 0;

        for (var i = 0; i < fourierTransform.length; i++) {
            let prevX = x;
            let prevY = y;

            let freq = i;
            let ampl = Math.sqrt(fourierTransform[i].Re * fourierTransform[i].Re + fourierTransform[i].Im * fourierTransform[i].Im);                
            let phase = Math.atan2(fourierTransform[i].Im, fourierTransform[i].Re);

            let arg = freq * this.#counter / fourierTransform.length * 2 * Math.PI + phase + rot;
            x += ampl * Math.cos(arg);
            y += ampl * Math.sin(arg);

            ctx.lineWidth = 1.0;
            ctx.strokeStyle = this.epicyclesColor;
            ctx.beginPath();
            ctx.arc(xOff + prevX, yOff + prevY, ampl, 0, Math.PI * 2);
            ctx.stroke();
           
            ctx.lineWidth = 0.5;
            ctx.strokeStyle = this.linesColor;
            ctx.beginPath();
            ctx.moveTo(xOff + prevX, yOff + prevY);
            ctx.lineTo(xOff + x, yOff + y);
            ctx.stroke();
        }

        return {x, y}
    }

    #loadAdditionalContent() {
        this.container.insertAdjacentHTML(
            'beforeend', '<br><button class="box" id="' + this.name + '_sinewave">sine</button>');

        // Sine Wave Button
        document.getElementById(this.name + '_sinewave').onclick = () => this.setPoints(this.#getSineWave());
    }

    #getSineWave() {
        let result = [];

        for (var i = 0; i < 100; i++) {
            result[i] = {
                x: i,
                y: 100 * Math.sin(i / 100 * 2 * Math.PI)
            };
        }

        return result;
    }

}