<template>
    <div id="post">
        <mnav id="mnav"></mnav>
        <div id="content">
            <div id="new-title"><p>New Theme</p></div>
            <form id="form" >
                    <div id="topic-group">
                        <span  id="category">
                                <select v-if="username == 'admin'" name="community_name" v-model="CommunityName" id="category-control" >
                                    <option value="muro">muro<span class="icon-arrow"></span></option>
                                    <option value="office">Office</option>
                                    <option v-bind:value="community_name" v-for="(community_name, index) in community_names" :key="index">
                                            {{community_name}}
                                    </option>
                                </select>
                                <select v-else name="community_name" v-model="CommunityName" id="category-control">
                                    <option value="muro">muro<span class="icon-arrow"></span></option>
                                    <option v-bind:value="community_name" v-for="(community_name, index) in community_names" :key="index">
                                            {{community_name}}
                                    </option>
                                </select>
                        </span>
                        <span id="title">
                                <input type="text" name="title" v-model="Title" placeholder="Please input title">
                        </span>
                    </div>    
                    <div id="new">
                                <textarea name="content" v-model="Content" placeholder="Write new content in markdown!"></textarea>
                    </div>
                    <div id="new">
                                <button type="submit" id="submit" @click="post" ><span class="tip"> Post </span></button>
                    </div>
            </form>
        </div>
    </div>
</template>

<script>
import axios from 'axios'
import Mnav from '../../components/nav/Mnav'
export default {
    name: 'post',
    components: {
        "mnav": Mnav
    },
    data () {
        return {
            username: '',
            community_names: '',
            CommunityName: '',
            Title: '',
            Content: ''
        }
    },
    mounted: function() {
        var signin_username = JSON.parse(sessionStorage.getItem('signin_user')).username
        this.username = signin_username
        var user_id = JSON.parse(sessionStorage.getItem('signin_user')).id
        axios.post('http://localhost:8001/api/community_names',{
            create_user_id: user_id
        })
        .then((response) => {
            this.community_names = response.data.community_names
            console.log(response.data.community_names)
        })
        .catch((e) => {
            console.log(e)
        })
    },
    methods: {
        post() {
            var community_name = this.CommunityName
            var title = this.Title
            var content = this.Content
            var user_id = JSON.parse(sessionStorage.getItem('signin_user')).id
            axios.post('http://localhost:8001/api/theme_new', {
                user_id: user_id,
                community_name: community_name,
                title: title,
                content: content
            })
            .then(response => {
                window.location.reload ( true )
                this.$router.push('/a/community/community_name')
            })
            .catch(e => {
                console.log(e)
            })
        }
    }
}
</script>

<style scoped>
#new-title {
    margin: 4px auto;
    width: 100%;
    line-height: 33px;
    border :1px solid #CAC1C1;
    background-color: #FFFFFF;
}
form #topic-group {
   margin: 11px 0 11px 0;
}
form #topic-group #category #category-control {
    background-color: #FFFFFF;
    border :1px solid #CAC1C1; /*Chrome和Firefox里面的边框是不一样的，所以复写了一下*/
    border: solid 1px #CAC1C1;
    padding-left: 9px;
}
form #topic-group #category #category-control, form #topic-group input {
    height: 30px;
}
form #new textarea {
    width:100%; 
    height: 444px;
}
form #new button {
    width:63px; 
    line-height:25px;
    background-color:rgb(255, 255, 255);
    border :1px solid #a39c9c;
}
@media only screen and (max-width: 600px) {
    #content  {
      margin: 0.5rem auto;
      width: 95%;
    }
    form #topic-group #category #category-control, form #topic-group input {
        width: 100%;
    }
}
@media only screen and (min-width: 600px) and (max-width: 1000px) {
    #content  {
        margin: 0 auto;
        width: 72%;
        padding-top: 77px;
    }
    form #topic-group input {
        width: 72%;
        float: right;
    }
}
@media only screen and (min-width: 1000px) {
  #content  {
      margin: 0 auto;
      width: 66%;
      padding-top: 77px;
  }
  form #topic-group input {
        width: 80%;
        float: right;
  }
}
</style>