import { useState } from 'react'
import './App.css'
import * as wasm from "../wasm-src/pkg/wasm_src.js";

function App() {
  const [file, setFile] = useState(null);
  const [buffer, setBuffer] = useState(null)
  const [textImg, setTextImg] = useState(null);
  const [numRows, setNumRows] = useState(50);
  

  function handleFileUpload(e) {
    console.log(e.target.files);
    wasm.greet(42);
    const next_file= URL.createObjectURL(e.target.files[0])
    setFile(next_file);
    fetch_buffer(next_file)

  }

  function print() {
    wasm.add1([1,2,3,4])
    console.log(file)
    console.log(buffer);
    
  }

  function fetch_buffer(next_file) {
    if (!next_file) {
      console.log("No file found")
      return null;
    }
    return (
      fetch(next_file)
      .then(function (response) {
        return response.blob();
      })
      .then(function (blob) {
        console.log('Blob Fetched')
        console.log(blob.size, blob.type)
        blob.arrayBuffer().then((res) => {
          console.log("Buffer Materialized")
          const next_buffer = new Uint8Array(res);
          setBuffer(next_buffer);
          process(next_buffer, numRows)

        })
    })
    );
  }

  function process(b, r) {
    if (!b) {
      console.log("Buffer not ready yes")
      return null;
    }
    let a = wasm.convertImage(b, r);
    console.log(a);
    setTextImg(a);
  }

  function handleNumInput (e) {
    if (e.target.value) {
      setNumRows(e.target.value)
      process(buffer, e.target.value)
    }

  }

  function handleStepButton(step) {
    const next_row = parseInt(numRows)+step
    setNumRows(next_row)
    process(buffer, next_row)
  }


  return (
    <>
      <div id='app-container'>
        <div id="header-div">
          <div id='inputs' >
            <h2>Image To ASCII Converter</h2>
            <input type="file" onChange={handleFileUpload} />
            <h3>Rows: </h3>
            <input type='numeric' on onChange={handleNumInput} min="1" value={numRows} />
            <button onClick={() => handleStepButton(1)} >+</button>
            <button onClick={() => handleStepButton(-1)} >-</button>
          </div>
        </div>
        <div id="images"> 
          <div id="real-image">
            <img id="src-image" src={file} />
          </div>
          <div id="text-image" style={{width: "100%", height: "100%"}}>
            <div className="display-linebreak" style={{font: (65/numRows).toString().slice(0,4) + 'vh "Courier New", monospace'}}>{textImg}</div>
          </div>
        </div>
      </div>
    </>
  )
}

export default App
