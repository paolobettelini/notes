class Fourier {

    /**
     * Computes the discrete Fourier transform of a given signal.
     *
     * @param {Number[]} f the signal
    */
    static dft(f) {
        var fourierTransform = [];
        
        for (var k = 0; k < f.length; k++) {
            var sumRe = 0;
            var sumIm = 0;

            for (var n = 0; n < f.length; n++) {
                var angle = 2 * Math.PI * k * n / f.length;
                sumRe += f[n] * Math.cos(angle);
                sumIm -= f[n] * Math.sin(angle);
            }

            fourierTransform[k] = {
                Re: sumRe / f.length,
                Im: sumIm / f.length
            };
        }

        return fourierTransform;
    }

}