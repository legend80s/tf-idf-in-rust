{
  const input = document.querySelector("#query");

  document.querySelector('#submit').addEventListener('submit', (event) => {
    console.log(input.value)
    event.preventDefault()
  })
}