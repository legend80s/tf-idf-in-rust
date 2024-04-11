{
  const input = document.querySelector("#query");

  document.querySelector('#submit').addEventListener('submit', (event) => {
    event.preventDefault()

    postJSON(input.value);
  })

  async function postJSON(data) {
    try {
      const response = await fetch("/api/search", {
        method: "POST",
        headers: {
          "Content-Type": "text/plain",
        },
        body: data,
      });
  
      const result = await response.text();
      console.log("Success:", result);
    } catch (error) {
      console.error("Error:", error);
    }
  }
}