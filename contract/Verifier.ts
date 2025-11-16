import { Field, Bool, Struct } from "o1js";
import { Groth16Proof, Groth16Verifier } from "o1js";

export class Verifier {
  static vk = {
    alpha: "ab4712cfba0d4fb366fbb7576c93061e331cb9fbb7a21d0d0452839bd5b7dc84",
    beta: "dd0f98ee7c9d37bea6ba29249daa2ceaa5b24ae3d41f5ef5b56748a4a2e60400ffed0f78e47befa3af13eaaa099cfdc4e250f995aa4e5901ff328683a36dfb17",
    gamma: "194d671c05afc77ad9e2a5a252316aa72f4e1816886e17e9373a9d145da9f42a801702865ac924f1fcd88a2afc4584000c129dd411a09f0f2dce2c67ef50edaf",
    delta: "53aa615cc492a090aa869ea3d22aba28921646ccd3ffc1ec3c1f6e8b8ac7792547c4a4d2d3e58e9e256ce8f1801286aee7b4f4bea44b4974a8a50ab66ab74723",
    gamma_abc: ["d3584f4a38108bfeafd20b9b3f64d08df802be420df30f2e0c6794e9b5e9bd9c", "bf80fa3a35cdab50e13fab2c4730d92e168d4c3672b83f76db22392790fe7c20"]
  };

  static verify(proof: Groth16Proof, publicInput: Field[]): Bool {
    return Groth16Verifier.verify(proof, Verifier.vk, publicInput);
  }
}
