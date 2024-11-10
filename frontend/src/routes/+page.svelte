<script lang="ts">
  // Send webcam video over webrtc to localhost:3000
  let video: HTMLVideoElement
  let rgb: HTMLSpanElement
  async function startVideo() {
    const stream = await navigator.mediaDevices.getUserMedia({ video: true })
    video.srcObject = stream
    // Here we initiate the WebRTC connection
    startWebRTCConnection(stream)
  }

  async function startWebRTCConnection(stream) {
    const socket = new WebSocket('ws://localhost:8765') // WebSocket server URL

    socket.onopen = () => {
      console.log('WebSocket connection established.')
    }

    socket.onerror = (error) => {
      console.log('WebSocket Error: ' + error)
    }

    socket.onclose = () => {
      console.log('WebSocket connection closed.')
    }

    async function setupWebcam() {
      const canvas = document.createElement('canvas')
      const context = canvas.getContext('2d')

      // Send video frames as raw binary data (image data)
      setInterval(() => {
        context.drawImage(video, 0, 0, canvas.width, canvas.height)
        const frameData = context.getImageData(
          0,
          0,
          canvas.width,
          canvas.height
        ).data

        // Send the frame data as binary via WebSocket
        socket.send(frameData.buffer) // Send raw ArrayBuffer
      }, 100) // 10 frames per second (adjust as needed)
    }

    setupWebcam()
  }
  startVideo();
</script>

<div
  class="flex h-screen max-h-screen w-full flex-col items-center justify-center"
>
  <!-- svelte-ignore a11y_media_has_caption -->
  <video id="webcam" class="rounded-xl" bind:this={video} autoplay></video>
  <p>RGB at (0,0): <span id="rgb" bind:this={rgb}></span></p>
</div>
