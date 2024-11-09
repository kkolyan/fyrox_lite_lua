
namespace FyroxLite
{
    internal static class FyroxImpls
    {

        internal static void_result on_init(NativeInstanceId thiz)
        {
            try
            {
                GetNodeScript(thiz).OnInit();
                return void_result.FromFacade();
            }
            catch (Exception e)
            {
                return void_result.FromFacadeError(e.ToString());
            }
        }

        internal static void_result on_start(NativeInstanceId thiz)
        {
            try
            {
                GetNodeScript(thiz).OnStart();
                return void_result.FromFacade();
            }
            catch (Exception e)
            {
                return void_result.FromFacadeError(e.ToString());
            }
        }

        internal static void_result on_deinit(NativeInstanceId thiz)
        {
            try
            {
                GetNodeScript(thiz).OnDeinit();
                return void_result.FromFacade();
            }
            catch (Exception e)
            {
                return void_result.FromFacadeError(e.ToString());
            }
        }

        internal static void_result on_update(NativeInstanceId thiz, float dt)
        {
            try
            {
                GetNodeScript(thiz).OnUpdate(dt);
                return void_result.FromFacade();
            }
            catch (Exception e)
            {
                return void_result.FromFacadeError(e.ToString());
            }
        }

        internal static void_result on_message(NativeInstanceId thiz, UserScriptMessage message)
        {
            try
            {
                var m = ObjectRegistry.Get(message.id);
                if (m == null)
                {
                    throw new Exception($"message not found for id {message.id}");
                }

                GetNodeScript(thiz).OnMessage(m);
                return void_result.FromFacade();
            }
            catch (Exception e)
            {
                return void_result.FromFacadeError(e.ToString());
            }
        }

        internal static void_result on_game_init(NativeInstanceId thiz, NativeString_optional initial_scene_override)
        {
            try
            {
                GetGlobalScript(thiz).OnGlobalInit(NativeString_optional.ToFacade(initial_scene_override));
                return void_result.FromFacade();
            }
            catch (Exception e)
            {
                return void_result.FromFacadeError(e.ToString());
            }
        }

        internal static void_result on_game_update(NativeInstanceId thiz)
        {
            try
            {
                GetGlobalScript(thiz).OnGlobalUpdate();
                return void_result.FromFacade();
            }
            catch (Exception e)
            {
                return void_result.FromFacadeError(e.ToString());
            }
        }

        internal static NativeInstanceId_result create_script_instance(NativeClassId thiz, NativePropertyValue_slice state, NativeHandle_optional node)
        {
            try
            {
                var instance = Activator.CreateInstance(thiz.GetCsClass());
                if (instance is NodeScript ns)
                {
                    var handle = NativeHandle_optional.ToFacade(node) ?? throw new Exception("node handle required for Node scripts, but not supplied");
                    ns._node = new Node(handle);
                }
                PropertySetters.SetProperties(instance, NativePropertyValue_slice.ToFacade(state));
                return NativeInstanceId_result.FromFacade(instance);
            }
            catch (Exception e)
            {
                return NativeInstanceId_result.FromFacadeError(e.ToString());
            }
        }

        internal static void dispose_message(UserScriptMessage message)
        {
            ObjectRegistry.Drop(message.id);
        }

        internal static void dispose_script(NativeInstanceId script)
        {
            ObjectRegistry.Drop(script.value);
        }

        private static NodeScript GetNodeScript(NativeInstanceId instanceId)
        {
            var o = ObjectRegistry.Get(instanceId.value);
            return o switch
            {
                NodeScript script => script,
                null => throw new Exception($"instance not found for id {instanceId.value}"),
                _ => throw new Exception($"instance is not a NodeScript {instanceId.value}")
            };
        }

        private static GlobalScript GetGlobalScript(NativeInstanceId instanceId)
        {
            var o = ObjectRegistry.Get(instanceId.value);
            return o switch
            {
                GlobalScript script => script,
                null => throw new Exception($"instance not found for id {instanceId.value}"),
                _ => throw new Exception($"instance is not a GlobalScript {instanceId.value}: {o.GetType()}")
            };
        }
    }
}