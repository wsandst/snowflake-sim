/**
 * This file contains various functions for interacting with WebGL
 */
import * as mat4 from 'gl-matrix/mat4';
import * as vec3 from 'gl-matrix/vec3';

// Draw the scene
export function draw(glCtx, programInfo, buffers, rotateX) {
    glCtx.clearColor(0.0, 0.0, 0.0, 1.0);  
    glCtx.clearDepth(1.0);            
    glCtx.enable(glCtx.DEPTH_TEST);
    glCtx.depthFunc(glCtx.LEQUAL);

    // Clear the canvas before we start drawing on it.
    glCtx.clear(glCtx.COLOR_BUFFER_BIT | glCtx.DEPTH_BUFFER_BIT);

    // Create a perspective matrix
    const fieldOfView = 45 * Math.PI / 180;   // in radians
    const aspect = glCtx.canvas.clientWidth / glCtx.canvas.clientHeight;
    const zNear = 0.1;
    const zFar = 100.0;
    const projectionMatrix = mat4.create();

    // note: glmatrix.js always has the first argument
    // as the destination to receive the result.
    mat4.perspective(projectionMatrix,
                    fieldOfView,
                    aspect,
                    zNear,
                    zFar);

    // Set the drawing position to the "identity" point, which is
    // the center of the scene.
    const modelViewMatrix = mat4.create();

    // Now move the drawing position a bit to where we want to
    // start drawing the square.

    mat4.translate(modelViewMatrix,     // destination matrix
                    modelViewMatrix,     // matrix to translate
                    [-0.0, 0.0, -6.0]);  // amount to translate

    mat4.rotateX(modelViewMatrix, modelViewMatrix, rotateX);
    mat4.rotateY(modelViewMatrix, modelViewMatrix, rotateX);

    // Tell WebGL how to pull out the positions from the position
    // buffer into the vertexPosition attribute.
    {
        const numComponents = 2;  // pull out 2 values per iteration
        const type = glCtx.FLOAT;    // the data in the buffer is 32bit floats
        const normalize = false;  // don't normalize
        const stride = 0;         // how many bytes to get from one set of values to the next
                                // 0 = use type and numComponents above
        const offset = 0;         // how many bytes inside the buffer to start from
        glCtx.bindBuffer(glCtx.ARRAY_BUFFER, buffers.position);
        glCtx.vertexAttribPointer(
            programInfo.attribLocations.vertexPosition,
            numComponents,
            type,
            normalize,
            stride,
            offset);
        glCtx.enableVertexAttribArray(
            programInfo.attribLocations.vertexPosition);
    }

    // Tell WebGL to use our program when drawing
    glCtx.useProgram(programInfo.program);

    // Set the shader uniforms
    glCtx.uniformMatrix4fv(
        programInfo.uniformLocations.projectionMatrix,
        false,
        projectionMatrix);
    glCtx.uniformMatrix4fv(
        programInfo.uniformLocations.modelViewMatrix,
        false,
        modelViewMatrix);
    {
        const offset = 0;
        const vertexCount = 4;
        glCtx.drawArrays(glCtx.TRIANGLE_STRIP, offset, vertexCount);
    }
}

// Initiate WebGL buffers
export function initBuffers(glCtx) {
    // Create a buffer for the square's positions.

    const positionBuffer = glCtx.createBuffer();

    // Select the positionBuffer as the one to apply buffer
    // operations to from here out.

    glCtx.bindBuffer(glCtx.ARRAY_BUFFER, positionBuffer);

    // Now create an array of positions for the square.

    const positions = [
        1.0,  1.0,
        -1.0,  1.0,
        1.0, -1.0,
        -1.0, -1.0,
    ];

    // Now pass the list of positions into WebGL to build the
    // shape. We do this by creating a Float32Array from the
    // JavaScript array, then use it to fill the current buffer.

    glCtx.bufferData(glCtx.ARRAY_BUFFER,
                new Float32Array(positions),
                glCtx.STATIC_DRAW);

    return {
        position: positionBuffer,
    };
}

// Compile a shader from source
function loadShader(glCtx, type, source) {
    const shader = glCtx.createShader(type);

    // Send the source to the shader object

    glCtx.shaderSource(shader, source);

    // Compile the shader program

    glCtx.compileShader(shader);

    // See if it compiled successfully

    if (!glCtx.getShaderParameter(shader, glCtx.COMPILE_STATUS)) {
        alert('An error occurred compiling the shaders: ' + glCtx.getShaderInfoLog(shader));
        glCtx.deleteShader(shader);
        return null;
    }

    return shader;
}

// Initiate the shaders
export function initShaderProgram(glCtx, vsSource, fsSource) {
    const vertexShader = loadShader(glCtx, glCtx.VERTEX_SHADER, vsSource);
    const fragmentShader = loadShader(glCtx, glCtx.FRAGMENT_SHADER, fsSource);

    // Create the shader program

    const shaderProgram = glCtx.createProgram();
    glCtx.attachShader(shaderProgram, vertexShader);
    glCtx.attachShader(shaderProgram, fragmentShader);
    glCtx.linkProgram(shaderProgram);

    // If creating the shader program failed, alert

    if (!glCtx.getProgramParameter(shaderProgram, glCtx.LINK_STATUS)) {
        alert('Unable to initialize the shader program: ' + glCtx.getProgramInfoLog(shaderProgram));
        return null;
    }

    return shaderProgram;
}