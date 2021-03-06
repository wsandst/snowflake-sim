/**
 * This file contains various functions for interacting with WebGL
 */
import * as mat4 from 'gl-matrix/mat4';
import * as vec3 from 'gl-matrix/vec3';

// Draw the scene
export function draw(glCtx, programInfo, buffers, vertexCount, offset, scale, color, canvasSize) {
    glCtx.clearColor(0.0, 0.0, 0.0, 1.0);  
    glCtx.clearDepth(1.0);            
    glCtx.enable(glCtx.DEPTH_TEST);
    glCtx.disable(glCtx.CULL_FACE);
    //glCtx.depthFunc(glCtx.LEQUAL);

    // Clear the canvas before we start drawing on it.
    glCtx.clear(glCtx.COLOR_BUFFER_BIT | glCtx.DEPTH_BUFFER_BIT);

    // Create a perspective matrix
    const fieldOfView = 45 * Math.PI / 180;   // in radians
    const aspect = glCtx.canvas.clientWidth / glCtx.canvas.clientHeight;
    const zNear = 0.1;
    const zFar = 100.0;
    const projectionMatrix = mat4.create();


    mat4.ortho(projectionMatrix, 0, canvasSize[0], 0, canvasSize[1], zNear, zFar);

    const modelViewMatrix = mat4.create();

    mat4.lookAt(modelViewMatrix, [0, 0, 6], [0, 0, 0], [0, 1, 0]);
    mat4.translate(modelViewMatrix, modelViewMatrix, [offset[0], offset[1], 0]);
    mat4.scale(modelViewMatrix, modelViewMatrix, [scale, scale, 0])
    

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

    // Setup color buffer
    {
        const numComponents = 4;
        const type = glCtx.FLOAT;
        const normalize = false;
        const stride = 0;
        const offset = 0;
        glCtx.bindBuffer(glCtx.ARRAY_BUFFER, buffers.color);
        glCtx.vertexAttribPointer(
            programInfo.attribLocations.vertexColor,
            numComponents,
            type,
            normalize,
            stride,
            offset);
        glCtx.enableVertexAttribArray(
            programInfo.attribLocations.vertexColor);
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
    glCtx.uniform4fv(
        programInfo.uniformLocations.hexColor,
        color,
    )
    {
        glCtx.drawArrays(glCtx.TRIANGLES, 0, vertexCount);
    }
}

// Initiate WebGL buffers
export function initBuffers(glCtx) {
    // Create a buffer for the square's positions.

    const positionBuffer = glCtx.createBuffer();
    const colorBuffer = glCtx.createBuffer();

    return {
        position: positionBuffer,
        color: colorBuffer,
    };
}

export function updateBufferData(glCtx, buffer, bufferData) {
    // Bind (select) the provided buffer
    glCtx.bindBuffer(glCtx.ARRAY_BUFFER, buffer);

    glCtx.bufferData(glCtx.ARRAY_BUFFER,
                new Float32Array(bufferData),
                glCtx.STATIC_DRAW);
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