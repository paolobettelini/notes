class Template extends InteractiveBox {

    constructor(name, container, height, width) {
        super(name, container, height, width)

        this.setPoints(this.#getDefaultPath());
    }

    draw(ctx) {
        this.clearCanvas();

        // draw function
    }

    onTimeTravel(value) {
        // onTimeTravel function
    }

    setPoints(points) {
        // setPoints function
    }

    #getDefaultPath() {
        var circle = [];
        for (var i = 0; i < 100; i++) {
            circle[i] = {
                x: 250 + 50 * Math.cos(Math.PI * 2 / 100 * i),
                y: 250 + 50 * Math.sin(Math.PI * 2 / 100 * i)
            }
        }
        return circle;
    }

}