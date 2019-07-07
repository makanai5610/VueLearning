Vue.component('list-item', {
    template: '<li>foo {{ contents }}</li>',
    data: function() {
        return {contents: 'bar'}
    }
})

new Vue({ el: '#example' })
