import axios = require('axios').default;

const mintApi = {
    requestMint: async function (amount) {
        const { data } = await axios.get(`${MINT_SERVER}/mint`, {
            params: {
                amount
            }
        })
        return data
    },
    mint: async function (payloads, paymentHash = '') {
        const { data } = await axios.post(`${MINT_SERVER}/mint`, payloads,
            {
                params: {
                    payment_hash: paymentHash
                }
            })
        return data
    },

    getKeys: async function () {
        const { data } = await axios.get(`${MINT_SERVER}/keys`)
        return data
    }
   
}

const mintFee = {
       requestFee: async function (fee) {
           const { data } = await axios.get(' ${FEE_SERVER/mint', {
                   params: {
                   fee
            }
           }) 
           return data
       }

module.exports = mintApi
module.exports = mintfEE
