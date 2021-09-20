import { dfinity_poc } from "../../declarations/dfinity_poc";

document.getElementById("clickMeBtn").addEventListener("click", async () => {
  const publicKey = document.getElementById("inputPublicKey").value.toString();
  const sign = document.getElementById("inputSign").value.toString();
  const message = document.getElementById("inputmessage").value.toString();

  document.getElementById("greeting").innerText = "loading... please wait...";

  // Interact with dfinity_dkg actor, calling the greet method
  const out = await dfinity_poc.store_message(publicKey, sign, message);
  console.log("verified successfully", out);

  document.getElementById("greeting").innerText = out;
});

document.getElementById("clickMeBtn2").addEventListener("click", async () => {
  const publicKey = document
    .getElementById("inputOldPublicKey")
    .value.toString();
  const sign = document.getElementById("inputSign2").value.toString();
  const message = document.getElementById("inputNewPublicKey").value.toString();

  document.getElementById("out2Section").innerText =
    "loading... please wait...";

  // await dfinity_poc.update_key(publicKey);
  // console.log("key updated successfully");
  // document.getElementById("out2Section").innerText = "public key updated successfully.";

  // Interact with dfinity_dkg actor, calling the greet method
  const out = await dfinity_poc.store_pub_key(publicKey, sign, message);
  console.log("verified successfully", out);

  document.getElementById("out2Section").innerText = out;
});

document
  .getElementById("setPublicKeySubmit")
  .addEventListener("click", async () => {
    const element = document.getElementById("setPublicKeyOutput");
    element.innerText = "please wait...";
    const publicKey = document
      .getElementById("inputSetPublicKey")
      .value.toString();

    // Interact with dfinity_dkg actor, calling the greet method
    const out = await dfinity_poc.update_key(publicKey);

    element.innerText = out;
  });

document
  .getElementById("fetchPublicKeySubmit")
  .addEventListener("click", async () => {
    const element = document.getElementById("fetchPublicKeyOutput");
    element.innerText = "getting public key... please wait...";

    // Interact with dfinity_dkg actor, calling the greet method
    const out = await dfinity_poc.get_public_key();

    element.innerText = out;
  });
