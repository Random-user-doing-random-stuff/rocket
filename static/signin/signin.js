const form = document.getElementById("userForm");

form.addEventListener("submit", async (event) => {
  event.preventDefault(); // Prevent form from reloading the page

  // Collect form data
  const formData = {
    name: document.getElementById("name").value,
    email: document.getElementById("email").value,
    password: document.getElementById("password").value,
    role: null, // You can modify this if you need to send a specific role
  };

  try {
    const response = await fetch("/api/users", {
      method: "POST",
      headers: {
        "Content-Type": "application/json", // Make sure it's JSON
      },
      body: JSON.stringify(formData), // Send the form data as JSON
    });

    if (response.ok) {
      alert("User added successfully!"); // Success message
      form.reset(); // Clear the form
    } else {
      const error = await response.json(); // Get error message
      console.error("Error response:", error);
      alert(`Failed to add user: ${error.message || "Unknown error"}`);
    }
  } catch (err) {
    console.error("Error:", err); // Log any other errors
    alert("An error occurred while adding the user.");
  }
});
