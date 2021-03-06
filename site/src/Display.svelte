
<script>
    // Display/render canvas component for viewing a Snowflake Simulation
    import { onMount } from 'svelte';
    import * as render from './render'

    // Import shader sources as strings
    import vertShaderSource from './shaders/vertex.vert'
    import fragShaderSource from './shaders/fragment.frag'

	let canvas;
	// WebGL internal state
	let glCtx;
	let shaderProgram;
	let buffers;
	let programInfo;
	let vertexCount = 0;
	// Drawing settings
	let hexWidth;
	let hexHeight
	let color = [0.5, 0.82, 0.96, 1.0];
	let offset = [0, 0];
	let scale = 1.0;

	onMount(() => {
		canvas.width = canvas.getBoundingClientRect().width;
		canvas.height = canvas.getBoundingClientRect().height;
		glCtx = canvas.getContext("webgl", {antialias: true});
        
        // Setup WebGL context
		shaderProgram = render.initShaderProgram(glCtx, vertShaderSource, fragShaderSource);

		programInfo = {
				program: shaderProgram,
				attribLocations: {
				vertexPosition: glCtx.getAttribLocation(shaderProgram, 'aVertexPosition'),
                vertexColor: glCtx.getAttribLocation(shaderProgram, 'aVertexColor'),
			},
				uniformLocations: {
				projectionMatrix: glCtx.getUniformLocation(shaderProgram, 'uProjectionMatrix'),
				modelViewMatrix: glCtx.getUniformLocation(shaderProgram, 'uModelViewMatrix'),
				hexColor: glCtx.getUniformLocation(shaderProgram, 'hexColor'),
			},
		};

		buffers = render.initBuffers(glCtx);

	});

    /**
     * Update the vertex position buffer
     * @param buffer vertex position buffer (x, y) * N
     */
    export function updatePositionBuffer(buffer) {
        render.updateBufferData(glCtx, buffers.position, buffer);
        vertexCount = buffer.length / 2;
    }

    /**
     * Update the vertex color buffer
     * @param buffer vertex color buffer (r, g, b, a) * N
     */
    export function updateColorBuffer(buffer) {
        render.updateBufferData(glCtx, buffers.color, buffer);
    }

	export function setSimSize(width, height) {
		hexWidth = width;
		hexHeight = height;
		updateCanvasSize();
	}

	export function setSimColor(hex) {
		var bigint = parseInt(hex, 16);
		var r = (bigint >> 16) & 255;
		var g = (bigint >> 8) & 255;
		var b = bigint & 255;
		color = [r,g,b,1.0];
		console.log(color);
		console.log(hex);
	}

	export function renderFrame() {
		if (canvas.width != canvas.getBoundingClientRect().width
				|| canvas.height != canvas.getBoundingClientRect().height) {
			updateCanvasSize();
		}
		render.draw(glCtx, programInfo, buffers, vertexCount, offset, scale, color, [canvas.width, canvas.height]);
	}

	/**
	 * Take a high-res screenshot (4000x4000)
	 * @return screenshot array
	 */
	export function screenshot() {
		// Take a 4000x4000 screenshot
		// Temporarily increase canvas size
		canvas.width = 4000;
		canvas.height = 4000;
		scale = (canvas.width-10)/(hexWidth*Math.sqrt(3));
		offset = [5, 35];
		glCtx.viewport(0, 0, canvas.width, canvas.height);
		render.draw(glCtx, programInfo, buffers, vertexCount, offset, scale, color, [canvas.width, canvas.height]);

		var offscreenCanvas = document.createElement("canvas");
		offscreenCanvas.width = canvas.width;
		offscreenCanvas.height = canvas.height;

		// Draw the OpenGL canvas to a 2D canvas
		var ctx = offscreenCanvas.getContext("2d");
		ctx.drawImage(canvas,0,0);

		// Save as array
		//let image = ctx.getImageData(0, 0, 2000, 2000);

		// Save as URL, open in new tab
		let url = offscreenCanvas.toDataURL('image/png');
		var newTab = window.open();
		newTab.document.body.innerHTML = 
			'<img src="' 
			+ url + 
			'" width="1000px" height="1000px"></img>';
			newTab.document.body.style.backgroundColor = "#202020"

		// Restore the canvas
		updateCanvasSize();
		renderFrame();
	}
	
	function updateCanvasSize() {
		// Update canvas dimensions if they changed
		canvas.width = canvas.getBoundingClientRect().width;
		canvas.height = canvas.getBoundingClientRect().height;
		scale = (canvas.width-10)/(hexWidth*Math.sqrt(3));
		offset = [5, 35];
		glCtx.viewport(0, 0, canvas.width, canvas.height);
	}

	function zoom(event) {
		// Disable zoom for now
		return;

		let x = event.offsetX / scale + offset[0];
		let y = (canvas.width - event.offsetY) / scale + offset[1];
		let zoomScale = (event.deltaY * -0.001);
		let newScale = scale + scale*zoomScale;
		let scaleDelta = newScale - scale;
		let offsetX = -(x * scaleDelta);
		let offsetY = -(y * scaleDelta);
		scale = newScale;
		offset = [offsetX, offsetY];
		renderFrame();
	}

</script>

<canvas bind:this={canvas} on:mousewheel={zoom}>

</canvas>

<style>
    @media only screen and (max-width: 480px) {
        canvas {
            width: 340px;
            height: 340px;
        }
    }

	@media only screen and (min-width: 480px) {
        canvas {
            width: 500px;
            height: 500px;
        }
    }

	@media only screen and (min-width: 1020px) {
        canvas {
            width: 650px;
            height: 650px;
        }
    }
</style>