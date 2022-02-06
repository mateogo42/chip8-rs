import { Emulator } from "chip8"

const canvas = document.getElementById("emulator")
canvas.width = 500
canvas.height = 300

const ctx = canvas.getContext("2d")
ctx.fillStyle = "black"
ctx.fillRect(0, 0, canvas.width, canvas.height)

const rom_input = document.getElementById("rom")
const emulator = Emulator.new()

rom_input.addEventListener("change", function () {
  const rom = rom_input.files[0]
  const fr = new FileReader()
  fr.readAsArrayBuffer(rom)

  fr.onload = function (e) {
    const romData = new Uint8Array(e.target.result)
    console.log(romData)
    const memory = emulator.load_rom_data(romData)
    console.log(memory)
    requestAnimationFrame(animationLoop)
  }
})

function animationLoop() {
  const newData = emulator.tick()
  const currentData = ctx.getImageData(0, 0, canvas.width, canvas.height)

  for (let i = 0; i < newData.length; i++) {
    if (newData[i] != 0) {
      currentData.data[i * 4] = 255
      console.log(i)
    } else {
      currentData.data[i * 4] = 0
    }
  }
  ctx.putImageData(currentData, 0, 0)
  // requestAnimationFrame(animationLoop)
}
