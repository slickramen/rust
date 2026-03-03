import init, { greet, add, Person, get_people, store_val_at_idx_zero, get_wasm_memory_buffer_pointer, read_wasm_memory_buffer_at_idx_one } from "../public/wasm/wasm_crate.js";

async function runWasm() {
    let rustWasm = await init();
    console.log(greet("Vite TypeScript with Wasm"));

    // Call the `add` function export from wasm, save the result
    const addResult = add(24, 24);

    // Set the result onto the body
    document.body.textContent = `addResult: ${addResult}`;

        // Get an array of Person objects from Rust
    let people = get_people();

    // Print the list
    console.log(people); 
    people.forEach(person => {
        console.log(`Name: ${person.name}, Age: ${person.age}`);
    });

    // Creating a new person and add them to the array
    let newPerson = new Person("David", 40);
    people.push(newPerson);

    console.log("After adding a new person:");
    people.forEach(person => {
        console.log(`Name: ${person.name}, Age: ${person.age}`);
    });

    // First, have wasm write to buffer at index 0
    store_val_at_idx_zero(24);

    // Next create a Uint8Array of wasm memory
    let wasmMemory = new Uint8Array(rustWasm.memory.buffer);

    // Get the pointer to the buffer that is within wasmMemory
    let bufferPointer = get_wasm_memory_buffer_pointer();

    // Read the written value at index zero of the buffer
    // by accessing the index of wasmMemory[bufferPointer + bufferIndex]
    console.log(wasmMemory[bufferPointer + 0]);

    // Write to index one of the buffer
    wasmMemory[bufferPointer + 1] = 10;

    // Have wasm read index one of the buffer
    // and return the result
    console.log(read_wasm_memory_buffer_at_idx_one());

}

runWasm();

