<script lang="ts">
	import webgazer from "webgazer";

  webgazer.setGazeListener(function(data, elapsedTime) {
    if (data == null) {
      return;
    }
    var xprediction = data.x; //these x coordinates are relative to the viewport
    var yprediction = data.y; //these y coordinates are relative to the viewport
    console.log(elapsedTime, xprediction, yprediction); //elapsed time is based on time since begin was called
  });
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p>
<button  type="button" on:click={() => webgazer.begin()}>Start</button>
<button  type="button" on:click={() => webgazer.pause()}>Stop</button>