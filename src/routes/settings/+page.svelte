<script lang="ts">
  import CheckBox from "@/components/CheckBox.svelte";
  import DirSelector from "@/components/DirSelector.svelte";
  import SelectBox from "@/components/SelectBox.svelte";
  import { setVariables, variables, type PartialConfig } from "./variables.svelte";
  import { onMount } from "svelte";

  let states: PartialConfig = {
    directory: {
      monitor: "",
      output: "",
    },
    behavior: {
      create_wrapper_folder: false,
      extracted_handling: {
        handling_mode: "NoAction",
        delete_permanently: false,
        move_destination: "",
      }
    },
  }
  let initialized = false;

  onMount(() => {
    states = variables
    initialized = true;
  });

  const onUpdate = () => {
    if (initialized) {
      setVariables(states);
    }
  }
</script>

<div class="container">
  <div>
    <section>
      <h2>
        ディレクトリ設定
      </h2>
      <section>
        <h3>
          監視するディレクトリ
        </h3>
        <div class="description">
          <p>
            AutoExtracter が監視するディレクトリを指定します。
          </p>
          <p>
            ダウンロードした圧縮ファイルをこのディレクトリへ移動することで、自動的にファイルの展開が行われます。
          </p>
        </div>
        <div class="controller">
          <DirSelector bind:value={states.directory.monitor} onUpdate={onUpdate}/>
        </div>
      </section>
      <section>
        <h3>
          展開先ディレクトリ
        </h3>
        <div class="description">
          <p>
            展開した内容を配置するディレクトリを指定します。
          </p>
        </div>
        <div class="controller">
          <DirSelector bind:value={states.directory.output} onUpdate={onUpdate}/>
        </div>
      </section>
    </section>
    <section>
      <h2>
        動作オプション
      </h2>
      <div class="controller">
        <CheckBox bind:value={states.behavior.create_wrapper_folder} onUpdate={onUpdate}>
          ダバァを抑制する（アーカイブ内が一つのフォルダでまとめられていない場合、圧縮ファイルと同じ名前のフォルダを作成し、その中に展開します）
        </CheckBox>
      </div>
      <section>
        <h3>
          展開済み圧縮ファイルの削除・移動
        </h3>
        <div class="description">
          <p>
            展開を終えた圧縮ファイルを自動的に削除するか、もしくは任意のディレクトリに移動させることができます。
          </p>
        </div>
        <div class="controller">
          <SelectBox bind:value={states.behavior.extracted_handling.handling_mode} onUpdate={onUpdate}
            options={[
              ["NoAction", "何もしない"],
              ["Delete", "自動的に削除する"],
              ["Move", "指定ディレクトリへ移動する"]
            ]}
          />
          <div class="{states.behavior.extracted_handling.handling_mode == "Delete" ? "" : "disabled"}">
            <CheckBox bind:value={states.behavior.extracted_handling.delete_permanently} onUpdate={onUpdate}>
              完全に削除する（チェックボックスが外れている場合はごみ箱へ移動します）
            </CheckBox>
          </div>
          <div class="{states.behavior.extracted_handling.handling_mode == "Move" ? "" : "disabled"}">
            <DirSelector bind:value={states.behavior.extracted_handling.move_destination} disabled={states.behavior.extracted_handling.handling_mode != "Move"} onUpdate={onUpdate}/>
          </div>
        </div>
      </section>
    </section>
    <div class="extra-space"></div>
  </div>
</div>

<style>
  .container {
    user-select: none;
    padding-left: 10px;
    overflow-y: scroll;
    max-height: calc(100vh - 100px);
  }

  .container > div {
    flex-direction: column;
    row-gap: 50px;
    box-shadow: -3px 0 3px #0461;
    display: flex;
  }

  .container > div > section {
    row-gap: 25px;
  }

  .controller {
    display: flex;
    flex-direction: column;
    row-gap: 3px;
  }

  section {
    position: relative;
    border-left: solid 5px #ddd;
    padding: 0 10px 10px;
    display: flex;
    flex-direction: column;
    row-gap: 15px;
    font-size: 0.8rem;
  }

  section h2 {
    font-size: 1rem;
    font-weight: 500;
  }

  section h3 {
    font-weight: 500;
  }

  section h2::before {
    content: "";
    border-left: solid 5px #999;
    padding: 0 10px;
    height: 20px;

    position: absolute;
    left: -4.5px;
  }

  section h3::before {
    content: "";
    border-left: solid 5px #abb;
    padding: 0 10px;
    height: 20px;

    position: absolute;
    left: -4.5px;
  }

  section div {
    text-indent: 0.3rem;
  }

  p {
    line-height: 1.2lh;
  }

  .extra-space {
    height: 200px;
  }

  .disabled {
    pointer-events: none;
    opacity: 0.3;
  }
</style>