<template>
  <section>
    <!-- header -->
    <div class="navbar">
      <div class="left">
        <span>{{ $t(this.$route.query.name) }}</span>
      </div>

      <div class="right">
        <span class="button-warp">
          <a
            :id="email"
            style="color: #1a7444"
            :href="'mailto:' + email"
            target="_blank"
            >{{ $t("modal.new_thread") }}</a
          >
          <Icon type="md-add" />
        </span>
      </div>
    </div>
    <div class="input-height" />
    <!-- Table button -->

    <List item-layout="vertical">
      <ListItem v-for="item in data" :key="item.subject">
        <ListItemMeta
          :avatar="item.avatar"
          :title="item.subject"
          :description="item.from_email"
        />
        <div v-html="item.body" />
        <template slot="action">
          <li>
            <!-- 此处直接用函数来表示 -->
            <!-- <router-link :to="{path: '/archive', query: {id: item.id}}">
                    More
                  </router-link> -->
            <span style="color: #1a7444" @click="handleModalSwitch">More</span>
          </li>
          <li>
            <!-- 此处使用超链接进行发送 然后在此处自动调用一个函数 将reply记录下来 在进行转发的时候将此值添加进去 然后我们在创建列表的时候 将Message-ID与refenrces相同的进行引用 放入同一个会话中 -->
            <a
              :id="email"
              style="color: #1a7444"
              :href="'mailto:' + email + '?subject=Re:' + item.subject"
              target="_blank"
              v-on:click="doChange"
              >Reply</a>
              <!-- <span type="text">Reply</span> -->
          </li>
        </template>
        <template slot="extra">
          {{ item.create_time | formatDate }}
        </template>
      </ListItem>
    </List>
  </section>
</template>

<script>
export default {
  name: "archive",
  components: {},
  data() {
    return {
      email: this.$route.query.email,
      id: "",
      name: this.$route.query.name,
      from_email: "",
      subject: "",
      message_id: "",
      body: "",
      in_reply_to: "",
      reference: "",

      data: [],
    };
  },
  filters: {
    formatDate: function (value) {
      let time = parseInt(value);
      let date = new Date(time * 1000);
      let y = date.getFullYear();
      let MM = date.getMonth() + 1;
      MM = MM < 10 ? "0" + MM : MM;
      let d = date.getDate();
      d = d < 10 ? "0" + d : d;
      let h = date.getHours();
      h = h < 10 ? "0" + h : h;
      let m = date.getMinutes();
      m = m < 10 ? "0" + m : m;
      let s = date.getSeconds();
      s = s < 10 ? "0" + s : s;
      return y + "-" + MM + "-" + d + " " + h + ":" + m + ":" + s;
    },
  },
  watch: {
    param(val) {
      this.getListData();
    },
  },
  computed: {},
  created() {
    this.getListData();
  },
  methods: {
    getListData() {
      this.$axios
        .get("/maillist/archive/list", {})
        .then((res) => {
          if (res.data.code == 0) {
            this.data = res.data.data;
          } else {
            this.$Message.error({
              content: this.$t("tip.request_fail_content"),
              duration: 3,
            });
          }
        })
        .catch((error) => {
          console.log(error);
          this.$Message.error({
            content: this.$t("tip.fault_content"),
            duration: 3,
          });
        });
    },
    doChange() {

    }
  },
};
</script>
<style lang="scss" scoped>
@import "./index.scss";
.select_type {
  width: 350px;

  /*滚动条整体部分*/
  ::v-deep .ivu-select-dropdown::-webkit-scrollbar {
    width: 6px;
    height: 10px;
  }
  /*滚动条的轨道*/
  ::v-deep .ivu-select-dropdown::-webkit-scrollbar-track {
    background-color: #ffffff;
  }
  /*滚动条里面的小方块，能向上向下移动*/
  ::v-deep .ivu-select-dropdown::-webkit-scrollbar-thumb {
    background-color: #ebebeb;
    border-radius: 5px;
    border: 1px solid #f1f1f1;
    box-shadow: inset 0 0 6px rgba(0, 0, 0, 0.3);
  }
  ::v-deep .ivu-select-dropdown::-webkit-scrollbar-thumb:hover {
    background-color: #a8a8a8;
  }
  ::v-deep .ivu-select-dropdown::-webkit-scrollbar-thumb:active {
    background-color: #787878;
  }
  /*边角，即两个滚动条的交汇处*/
  ::v-deep .ivu-select-dropdown::-webkit-scrollbar-corner {
    background-color: #ffffff;
  }
}
</style>