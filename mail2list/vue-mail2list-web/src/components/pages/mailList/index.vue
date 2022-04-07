<template>
  <section>
    <!-- header -->
    <div class="navbar">
      <div class="left">
        <span>{{ $t("sidebar.maillist") }}</span>
      </div>
    </div>
    <!-- search -->
    <div class="input-height">
      <!-- <Input
        suffix="ios-search"
        v-model="param"
        :placeholder="$t('modal.placeholder')"
        style="width: 300px"/> -->
    </div>
    <!-- Table button -->
    <Table border :columns="columns" :data="tableData">
      <template slot-scope="{ row }" slot="action">
        <Tooltip
          v-for="(item, index) in promptContent"
          :key="index"
          :content="item.content"
          placement="top-start"
        >
          <span class="button-warp" @click="handleButtonSelect(row, index + 1)">
            <Icon :type="item.icon" />
          </span>
        </Tooltip>
      </template>
    </Table>

    <Modal
      v-model="isOpen"
      :title="
        id
          ? $t('maillist_columns.create_title')
          : $t('maillist_columns.create_title')
      "
      :ok-text="$t('modal.ok_text')"
      :cancel-text="$t('modal.cancel_text')"
      @on-ok="handleSaveUpdateData"
    >
      <!-- <div class="modal-warp Schedule">
        <div class="item">
          <label>{{$t('maillist_columns.username')}}：</label>
          <Input
              show-word-limit
              maxlength="100"
              v-model="username"
              :placeholder="$t('modal.placeholder')"
              style="width: 350px" />
        </div>
        <div class="item">
          <label>{{$t('maillist_columns.driverMemory')}}：</label>
          <Input
              show-word-limit
              maxlength="100"
              v-model="driverMemory"
              :placeholder="$t('modal.placeholder')"
              style="width: 350px"/>
        </div>
        <div class="item">
          <label>{{$t('maillist_columns.executorNumber')}}：</label>
          <Input
              show-word-limit
              maxlength="100"
              v-model="executorNumber"
              :placeholder="$t('modal.placeholder')"
              style="width: 350px"/>
        </div>
        <div class="item">
          <label>{{$t('maillist_columns.executorMemory')}}：</label>
          <Input
              show-word-limit
              maxlength="100"
              v-model="executorMemory"
              :placeholder="$t('modal.placeholder')"
              style="width: 350px"/>
        </div>
        <div class="item">
          <label>{{$t('maillist_columns.executorCores')}}：</label>
          <Input
              show-word-limit
              maxlength="100"
              v-model="executorCores"
              :placeholder="$t('modal.placeholder')"
              style="width: 350px"/>
        </div>
        <div class="item">
          <label>GlobalVariable：</label>
          <Select class="select_type" v-model="fieldType" multiple>
            <Option v-for="item in typeList" :value="item.id" :key="item.id">{{ item.name }}</Option>
          </Select>
        </div>
        <div class="item">
          <label class="self">{{$t('maillist_columns.description')}}：</label>
          <Input
            v-model="description"
            type="textarea"
            :rows="4"
            :placeholder="$t('modal.placeholder')"
            style="width: 350px"/>
        </div>
      </div> -->
      <div class="modal-warp">
        <Form
          ref="formValidate"
          :model="formValidate"
          :rules="ruleValidate"
          :label-width="80"
        >
          <FormItem :label="$t('maillist_columns.username')">
            <Input
              v-model="formValidate.username"
              show-word-limit
              maxlength="100"
              :placeholder="$t('modal.placeholder')"
              style="width: 340px"
            />
          </FormItem>

          <FormItem :label="$t('maillist_columns.email')" prop="mail">
            <Input
              v-model="formValidate.mail"
              show-word-limit
              maxlength="100"
              :placeholder="$t('modal.placeholder')"
              style="width: 340px"
            />
          </FormItem>
        </Form>
      </div>
    </Modal>
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
      username: "",

      formValidate: {
        mail: "",
      },
      ruleValidate: {
        mail: [
          {
            required: true,
            message: "Mailbox cannot be empty",
            trigger: "blur",
          },
          { type: "email", message: "Incorrect email format", trigger: "blur" },
        ],
      },

      promptContent: [
        {
          content: "Subscribe",
          icon: "ios-redo",
        },
        {
          content: "Archive",
          icon: "ios-archive",
        },
      ],
    };
  },
  watch: {
    isOpen(state) {
      if (!state) {
        this.handleReset();
      } else if (state && this.id === "") {
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
      this.archive = "";
      this.username = "";
    },

    handleButtonSelect(row, key) {
      switch (key) {
        case 2:
          this.$event.emit("crumb", [
            { name: "maillist", path: "/maillist" },
            { name: "archive", path: "/archive" },
          ]);
          this.$router.push({
            path: "/archive",
            // query: {id:row.id, email:row.email},
            query: {
              // src: "/archive/page/flow/mxGraph/index.html?load=" + row.id,
              name: row.name,
              email: row.email,
            },
          });
          break;
        case 1:
          this.getRowData(row);
          break;
        default:
          break;
      }
    },

    // add / update
    handleSaveUpdateData() {
      let data = {
        username: this.formValidate.username,
        email: this.formValidate.mail,
        name: this.name,
      };
      //update
      // data.id = this.id;
      this.$axios
        .post("/maillist/menu/subscribe", JSON.stringify(data), {
          headers: {
            "content-type": "application/json",
          },
        })
        .then((res) => {
          if (res.data.code === 0) {
            this.$Modal.success({
              title: this.$t("tip.title"),
              // content `${this.data.data}`,
              content: `${res.data.data} `,
            });
            this.isOpen = false;
            this.handleReset();
            this.getTableData();
          } else {
            this.$Message.error({
              content: `${this.name} ` + this.$t("tip.update_fail_content"),
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
              duration: 3,
            });
          } else {
            this.tableData = res.data.data;
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