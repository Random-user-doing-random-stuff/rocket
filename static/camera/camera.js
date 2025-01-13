const videoElement = document.getElementById("video");
const stopButton = document.getElementById("stopButton");
const startButton = document.getElementById("startButton");
let mediaStream = null; // To store the stream globally

const startCamera = async () => {
  try {
    mediaStream = await navigator.mediaDevices.getUserMedia({ video: true });
    videoElement.srcObject = mediaStream;
    console.log("Camera started");
  } catch (err) {
    console.error("Error accessing the camera:", err);
  }
};

const stopCamera = () => {
  if (mediaStream) {
    // Stop all tracks in the media stream
    mediaStream.getTracks().forEach((track) => {
      console.log(`Stopping track: ${track.kind}`); // Debugging info
      track.stop();
    });

    // Clear the video feed and release resources
    videoElement.srcObject = null;
    mediaStream = null; // Release the media stream
    console.log("Camera stopped and resources released");
  } else {
    console.warn("No media stream to stop");
  }
};

// Button event listeners
startButton.addEventListener("click", startCamera);
stopButton.addEventListener("click", stopCamera);
