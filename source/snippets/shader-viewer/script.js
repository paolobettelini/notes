let startTime = Date.now();
let gl;

function initGL() {
  const canvas = document.getElementById('outputCanvas');
  gl = canvas.getContext('webgl');
}

function runShader() {
  initGL();

  const vertexShaderCode = `
    void main() {
      gl_Position = vec4(0.0, 0.0, 0.0, 1.0);
      gl_PointSize = 500.0;
    }
  `;

  const fragmentShaderCode = document.getElementById('glslCode').value;

  const vertexShader = gl.createShader(gl.VERTEX_SHADER);
  gl.shaderSource(vertexShader, vertexShaderCode);
  gl.compileShader(vertexShader);

  const fragmentShader = gl.createShader(gl.FRAGMENT_SHADER);
  gl.shaderSource(fragmentShader, fragmentShaderCode);
  gl.compileShader(fragmentShader);

  const shaderProgram = gl.createProgram();
  gl.attachShader(shaderProgram, vertexShader);
  gl.attachShader(shaderProgram, fragmentShader);
  gl.linkProgram(shaderProgram);
  gl.useProgram(shaderProgram);

  const timeLocation = gl.getUniformLocation(shaderProgram, 'u_time');

  function render() {
    const currentTime = Date.now();
    const elapsedTime = (currentTime - startTime) / 1000.0;

    gl.clearColor(0.0, 0.0, 0.0, 1.0);
    gl.clear(gl.COLOR_BUFFER_BIT);

    gl.uniform1f(timeLocation, elapsedTime);

    gl.drawArrays(gl.POINTS, 0, 1);

    // does this keep going when i repress runShader again?
    requestAnimationFrame(render);
  }

  render();
}
