// source: https://github.com/paritytech/subxt/blob/master/examples/wasm-example/index.js
/**
 * The `@polkadot/extension-dapp` package can be dynamically imported.
 * Usually it is wise to use a package manager like npm or yarn to install it as a dependency.
 *
 * The `getPolkadotJsExtensionMod` closure returns the `@polkadot/extension-dapp` module on demand.
 */
let getPolkadotJsExtensionMod = (() => {
    let mod = null;

    // initialize `@polkadot/extension-dapp` module on page load
    let initPromise = (async () => {
        mod = await import(
            "https://cdn.jsdelivr.net/npm/@polkadot/extension-dapp@0.46.3/+esm"
            );
    })();

    // return a function that waits for initialization to be finished, in case mod is not initialized yet.
    return async () => {
        if (mod == null) {
            await initPromise;
        }
        return mod;
    };
})();

/**
 *  Queries wallets from browser extensions like Talisman and the Polkadot.js extension for user accounts.
 *
 *  @returns a json string that contains all the accounts that were found.
 */
async function getAccounts() {
    const extensionMod = await getPolkadotJsExtensionMod();
    await extensionMod.web3Enable("Subxt Example App");
    const allAccounts = await extensionMod.web3Accounts();
    const accountObjects = allAccounts.map((account) => ({
        name: account.meta.name, // e.g. "Alice"
        source: account.meta.source, // e.g. "talisman", "polkadot-js"
        ty: account.type, // e.g. "sr25519"
        address: account.address // e.g. "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
    }));
    console.log(accountObjects);
    return JSON.stringify(accountObjects);
}

/**
 * Signs a payload using the Polkadot.js extension.
 * @param dataAsStr the payload to be signed, as a string
 * @param source the source of the account to sign with, e.g. "polkadot-js"
 * @param address the address of the account to sign with
 * @returns a json string that contains the signature
 */
async function signRaw(dataAsStr, source, address) {
    const extensionMod = await getPolkadotJsExtensionMod();
    const injector = await extensionMod.web3FromSource(source);
    const signRaw = injector?.signer?.signRaw;
    if (!!signRaw) {
        const {signature} = await signRaw({
            address,
            data: dataAsStr,
            type: "bytes"
        });
        console.log("data is:", dataAsStr)
        console.log("signature is:", signature)
        return signature;
    } else {
        throw "The extension's injector does not have a `signRaw` function on its `signer`";
    }
}
