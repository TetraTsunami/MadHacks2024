<script lang="ts">
	import { onMount } from 'svelte';
  onMount(async () => {
    eval(await fetch("./GazeCloudAPI.js").then((res) => res.text()));
  });
  onMount(() => {
    setTimeout(() => {
      console.log("log", GazeCloudAPI);
      GazeCloudAPI.OnCalibrationComplete = function(){
        console.log('gaze Calibration Complete')
      }         
      GazeCloudAPI.OnCamDenied = function(){ 
        console.log('camera access denied')  
      }         
      GazeCloudAPI.OnResult = console.log;
      GazeCloudAPI.OnError = function(msg: any){ console.log('err: ' + msg) }
      GazeCloudAPI.UseClickRecalibration = true;
    }, 1000);    
  });
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p>
<button  type="button" on:click={() => GazeCloudAPI.StartEyeTracking()}>Start</button>
<button  type="button" on:click={() => GazeCloudAPI.StopEyeTracking()}>Stop</button>