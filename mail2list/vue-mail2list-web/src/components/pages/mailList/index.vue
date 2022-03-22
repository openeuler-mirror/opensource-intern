<template>
  <section>
    <!-- header -->
    <div class="navbar">
      <div class="left">
        <span>{{$t("sidebar.maillist")}}</span>
      </div>
    </div>
    <!-- search -->
    <div class="input">
      <Input
        suffix="ios-search"
        v-model="param"
        :placeholder="$t('modal.placeholder')"
        style="width: 300px"/>
    </div>
    <!-- Table button -->
    <Table border :columns="columns" :data="tableData">
      <template slot-scope="{ row }" slot="action">
        <Tooltip v-for="(item, index) in promptContent" :key="index" :content="item.content" placement="top-start">
            <span class="button-warp" @click="handleButtonSelect(row,index+1)">
              <Icon :type="item.icon" />
            </span>
        </Tooltip>
      </template>
    </Table>
  </section>
</template>

<script>
export default {
  name: "maillist",
  components: {},
  data() {
    return {
      isOpen: false,
      isTemplateOpen: false,
      tableData: [],

      param: "",
      templateName: "",

      row: null,
      id: null,
      name: "",
      description: "",
      email: "",
      archive: "",

      formValidate: {
        mail: ''
      },
      ruleValidate: {
        mail: [
          { required: true, message: 'Mailbox cannot be empty', trigger: 'blur' },
          { type: 'email', message: 'Incorrect email format', trigger: 'blur' }
        ]
      },

      promptContent: [
        {
          content: 'Subscribe',
          icon: 'ios-redo'
        },{
          content: 'Archive',
          icon: 'ios-archive'
        }
      ]
    };
  },
  watch: {
    isOpen(state) {
      if (!state) {
        this.handleReset();
      }else if (state && this.id === ''){
        this.getGlobalList(false);
      }
    },
    param() {
      this.getTableData();
    },
  },
  computed: {
    columns() {
      return [
        {
          title: this.$t("maillist_columns.name"),
          key: "name",
          sortable: true,
        },
        {
          title: this.$t("maillist_columns.email"),
          key: "email",
        },
        {
          title: this.$t("maillist_columns.Archive"),
          key: "archive",
        },
        {
          title: this.$t("maillist_columns.description"),
          key: "description",
        },
        {
          title: this.$t("maillist_columns.action"),
          slot: "action",
          width: 350,
          align: "center",
        },
      ];
    },
  },
  created() {
    this.getTableData();
  },
  methods: {
    handleReset() {
      this.id = null;
      this.row = null;
      this.name = "";
      this.description = "";
      this.email = "";
      this.archive = ""
    },

    handleButtonSelect(row, key) {
      switch (key) {
        case 2:
          window.open('https://mailweb.openeuler.org/hyperkitty/list/' + row.name.toLowerCase() + '@openeuler.org/');
          break;
        case 1:
          window.open('https://mailweb.openeuler.org/postorius/lists/' + row.name.toLowerCase() + '@openeuler.org/');
          break;
        default:
          break;
      }
    },

    getRowData(row) {
      this.$event.emit("loading", true);
      this.$axios
        .get(`/maillist/menu/getListById/${row.id}`)
        .then((res) => {
          this.$event.emit("loading", false);
          if (res.data.code === 0) {
            let maillist = res.data.data;
            this.id = maillist.id;
            this.name = maillist.name;
            this.description = maillist.description;
            this.email = maillist.email;
            this.archive = maillist.archive;
            this.isOpen = true;
          } else {
            this.$Modal.success({
              title: this.$t("tip.title"),
              content: this.$t("tip.request_fail_content"),
            });
          }
        })
        .catch((error) => {
          console.log(error);
        });
    },


    getTableData() {
      this.$axios
        .get("/maillist/menu/list")
        .then((res) => {
          if (res.data.code != 0) {
            this.$Message.error({
              content: this.$t("tip.request_fail_content"),
              duration: 3
            });
          } else {
            this.tableData = res.data.data;
          }
        })
        .catch((error) => {
          console.log(error);
          this.$Message.error({
            content: this.$t("tip.fault_content"),
            duration: 3
          });
        });
    },
  },
};
</script>
<style lang="scss" scoped>
@import "./index.scss";
.select_type{
  width: 350px;

  /*滚动条整体部分*/
  ::v-deep .ivu-select-dropdown::-webkit-scrollbar {
    width: 6px;
    height: 10px;
  }
  /*滚动条的轨道*/
  ::v-deep .ivu-select-dropdown::-webkit-scrollbar-track {
    background-color: #FFFFFF;
  }
  /*滚动条里面的小方块，能向上向下移动*/
  ::v-deep .ivu-select-dropdown::-webkit-scrollbar-thumb {
    background-color: #ebebeb;
    border-radius: 5px;
    border: 1px solid #F1F1F1;
    box-shadow: inset 0 0 6px rgba(0,0,0,.3);
  }
  ::v-deep .ivu-select-dropdown::-webkit-scrollbar-thumb:hover {
    background-color: #A8A8A8;
  }
  ::v-deep .ivu-select-dropdown::-webkit-scrollbar-thumb:active {
    background-color: #787878;
  }
  /*边角，即两个滚动条的交汇处*/
  ::v-deep .ivu-select-dropdown::-webkit-scrollbar-corner {
    background-color: #FFFFFF;
  }

}

</style>