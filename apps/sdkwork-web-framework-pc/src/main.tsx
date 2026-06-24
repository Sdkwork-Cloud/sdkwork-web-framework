import { StrictMode, useEffect, useState } from "react";

import { createRoot } from "react-dom/client";

import {

  useWebFrameworkAdmin,

  type WebFrameworkAdminTab,

} from "./hooks/useWebFrameworkAdmin";

import "./styles.css";



function App() {

  const { loadTab, savePayload, heartbeatNode, deleteNode, visibleTabs, tabLabels } =
    useWebFrameworkAdmin();

  const [tab, setTab] = useState<WebFrameworkAdminTab>("defaults");

  const [environment, setEnvironment] = useState("prod");

  const [json, setJson] = useState("{}");

  const [output, setOutput] = useState("加载中…");

  const [error, setError] = useState<string | null>(null);



  async function refresh() {

    setError(null);

    try {

      const data = await loadTab(tab, environment);

      setOutput(JSON.stringify(data, null, 2));

    } catch (err) {

      setError(err instanceof Error ? err.message : String(err));

    }

  }



  useEffect(() => {

    if (!visibleTabs.includes(tab)) {

      setTab(visibleTabs[0] ?? "defaults");

    }

  }, [visibleTabs, tab]);



  useEffect(() => {

    void refresh();

  }, [tab, environment]);



  async function save() {

    setError(null);

    try {

      const payload = JSON.parse(json);

      await savePayload(tab, payload);

      await refresh();

    } catch (err) {

      setError(err instanceof Error ? err.message : String(err));

    }

  }



  async function heartbeatNodeAction() {

    setError(null);

    try {

      const payload = JSON.parse(json) as { node_id?: string };

      if (!payload.node_id) {

        throw new Error("JSON 需包含 node_id");

      }

      await heartbeatNode(payload.node_id);

      await refresh();

    } catch (err) {

      setError(err instanceof Error ? err.message : String(err));

    }

  }



  async function deleteNodeAction() {

    setError(null);

    try {

      const payload = JSON.parse(json) as { node_id?: string };

      if (!payload.node_id) {

        throw new Error("JSON 需包含 node_id");

      }

      await deleteNode(payload.node_id);

      await refresh();

    } catch (err) {

      setError(err instanceof Error ? err.message : String(err));

    }

  }



  return (

    <div className="layout" data-testid="web-framework-console">

      <header>

        <h1>SDKWork Web Framework Console</h1>

        <p>分布式运行时治理：CORS / 流控 / 租户配置 / 控制面节点</p>

      </header>

      <nav>

        {visibleTabs.map((id) => (

          <button

            key={id}

            className={tab === id ? "active" : ""}

            onClick={() => setTab(id)}

            type="button"

          >

            {tabLabels[id]}

          </button>

        ))}

      </nav>

      <section className="toolbar">

        <label>

          环境

          <select value={environment} onChange={(e) => setEnvironment(e.target.value)}>

            <option value="dev">dev</option>

            <option value="test">test</option>

            <option value="prod">prod</option>

          </select>

        </label>

        <button type="button" onClick={() => void refresh()}>

          刷新

        </button>

        {["cors", "rateLimit", "tenant", "nodes"].includes(tab) && (

          <>

            <button type="button" onClick={() => void save()}>

              保存 JSON

            </button>

            {tab === "nodes" && (

              <>

                <button type="button" onClick={() => void heartbeatNodeAction()}>

                  节点心跳

                </button>

                <button type="button" onClick={() => void deleteNodeAction()}>

                  删除节点

                </button>

              </>

            )}

          </>

        )}

      </section>

      {error && <div className="error">{error}</div>}

      <div className="panels">

        {["cors", "rateLimit", "tenant", "nodes"].includes(tab) && (

          <textarea

            value={json}

            onChange={(e) => setJson(e.target.value)}

            placeholder="编辑 upsert JSON"

            rows={12}

          />

        )}

        <pre>{output}</pre>

      </div>

    </div>

  );

}



createRoot(document.getElementById("root")!).render(

  <StrictMode>

    <App />

  </StrictMode>,

);


