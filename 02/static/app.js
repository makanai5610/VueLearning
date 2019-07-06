var items = [
    {
        name: '鉛筆',
        price: 300,
        quantity: 0
    },
    {
        name: 'ノート',
        price: 400,
        quantity: 0
    },
    {
        name: '消しゴム',
        price: 500,
        quantity: 0
    }
]

var vm = new Vue({
    el: '#b-button',
    data: {
        loggedInButton: 'ログイン済のため購入できます。',
        canBuy: true
    }
})

window.vm = vm
