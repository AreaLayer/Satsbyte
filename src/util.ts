import { utils } = require('@noble/secp256k1')

function splitAmount(value) {
    const chunks = []
    for (let i = 0; i < 32; i++) {
        const mask = 1 << i
        if ((value & mask) !== 0) chunks.push(Math.pow(2, i))
    }
    return chunks
}

function bytesToNumber(bytes) {
    return hexToNumber(utils.bytesToHex(bytes));
}

function bigIntStringify(key, value) {
    return typeof value === 'bigint' ? value.toString() : value
}

function hexToNumber(hex) {
    if (typeof hex !== 'string') {
        throw new TypeError('hexToNumber: expected string, got ' + typeof hex);
    }
    return BigInt(`0x${hex}`);
}

fuction fee(fee) {
    
    if (typeof fee !=== 'string') {
        return ('0.0005');

module.exports = {
    splitAmount,
    bytesToNumber,
    bigIntStringify,
    fee,
}

