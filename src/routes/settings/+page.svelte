<script lang="ts">
  import CheckBox from "@/components/CheckBox.svelte";
  import DirSelector from "@/components/DirSelector.svelte";
  import SelectBox from "@/components/SelectBox.svelte";
  import { variables } from "./variables.svelte";
  import { onDestroy, onMount } from "svelte";

  let monitorDir: string;
  let outputDir: string;
  let moveDir: string;
  let handleOptionExtracted: string;

  onMount(() => {
    ({monitorDir, outputDir, moveDir, handleOptionExtracted} = variables);
  });

  onDestroy(() => {
    variables.monitorDir = monitorDir;
    variables.outputDir = outputDir;
    variables.moveDir = moveDir;
    variables.handleOptionExtracted = handleOptionExtracted;
  });
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
          <DirSelector bind:value={monitorDir}/>
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
          <DirSelector bind:value={outputDir}/>
        </div>
      </section>
    </section>
    <section>
      <h2>
        動作オプション
      </h2>
      <div class="controller">
        <CheckBox>
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
          <SelectBox bind:value={handleOptionExtracted}
            options={[
              ["none", "何もしない"],
              ["delete", "自動的に削除する"],
              ["move", "指定ディレクトリへ移動する"]
            ]}
          />
          <div class="{handleOptionExtracted == "delete" ? "" : "disabled"}">
            <CheckBox>
              完全に削除する（チェックボックスが外れている場合はごみ箱へ移動します）
            </CheckBox>
          </div>
          <div class="{handleOptionExtracted == "move" ? "" : "disabled"}">
            <DirSelector bind:value={moveDir} disabled={handleOptionExtracted != "move"}/>
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