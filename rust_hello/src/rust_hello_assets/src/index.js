import { rust_hello } from "../../declarations/rust_hello";

document.getElementById("clickMeBtn").addEventListener("click", async () => {
  const publicKey = document.getElementById("inputPublicKey").value.toString();
  const sign = document.getElementById("inputSign").value.toString();
  const message = document.getElementById("inputmessage").value.toString();

  document.getElementById("greeting").innerText = "loading... please wait...";

  await rust_hello.update_key(publicKey);
  console.log("key updated successfully");
  document.getElementById("greeting").innerText = "public key updated successfully.. Please wait for verification...";

  // Interact with dfinity_dkg actor, calling the greet method
  const out = await rust_hello.store_message(publicKey, sign, message);
  console.log("verified successfully", out);

  document.getElementById("greeting").innerText = out;
});


document.getElementById("clickMeBtn2").addEventListener("click", async () => {
  const publicKey = document.getElementById("inputOldPublicKey").value.toString();
  const sign = document.getElementById("inputSign2").value.toString();
  const message = document.getElementById("inputNewPublicKey").value.toString();

  document.getElementById("out2Section").innerText = "loading... please wait...";

  // await rust_hello.update_key(publicKey);
  // console.log("key updated successfully");
  // document.getElementById("out2Section").innerText = "public key updated successfully.";

  // Interact with dfinity_dkg actor, calling the greet method
  const out = await rust_hello.store_pub_key(publicKey, sign, message);
  console.log("verified successfully", out);

  document.getElementById("out2Section").innerText = out;
});


document.getElementById("getPublicKey").addEventListener("click", async () => {

  document.getElementById("greeting").innerText = "getting public key... please wait...";

  // Interact with dfinity_dkg actor, calling the greet method
  const out = await rust_hello.get_public_key();

  document.getElementById("greeting").innerText = out;
});
