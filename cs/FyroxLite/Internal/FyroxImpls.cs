using App01;
using FyroxLite.Internal;

namespace FyroxLite
{
    internal static class FyroxImpls
    {

        internal static void on_init(NativeInstanceId thiz)
        {
            try
            {
                GetNodeScript(thiz).OnInit();
            }
            catch (Exception e)
            {
                HandleException(e);
            }
        }

        internal static void on_start(NativeInstanceId thiz)
        {
            try
            {
                GetNodeScript(thiz).OnStart();
            }
            catch (Exception e)
            {
                HandleException(e);
            }
        }

        internal static void on_deinit(NativeInstanceId thiz)
        {
            try
            {
                GetNodeScript(thiz).OnDeinit();
            }
            catch (Exception e)
            {
                HandleException(e);
            }
        }

        internal static void on_update(NativeInstanceId thiz, float dt)
        {
            try
            {
                GetNodeScript(thiz).OnUpdate(dt);
            }
            catch (Exception e)
            {
                HandleException(e);
            }
        }

        internal static void on_message(NativeInstanceId thiz, UserScriptMessage message)
        {
            try
            {
                var m = ObjectRegistry.Get(message.id);
                if (m == null)
                {
                    throw new Exception($"message not found for id {message.id}");
                }

                GetNodeScript(thiz).OnMessage(thiz);
            }
            catch (Exception e)
            {
                HandleException(e);
            }
        }

        internal static void on_game_init(NativeInstanceId thiz)
        {
            try
            {
                GetGlobalScript(thiz).OnGlobalInit();
            }
            catch (Exception e)
            {
                HandleException(e);
            }
        }

        internal static void on_game_update(NativeInstanceId thiz)
        {
            try
            {
                GetGlobalScript(thiz).OnGlobalUpdate();
            }
            catch (Exception e)
            {
                HandleException(e);
            }
        }

        internal static NativeInstanceId_result create_script_instance(NativeClassId thiz, NativePropertyValue_slice state)
        {
            try
            {
                var instance = Activator.CreateInstance(thiz.GetType());
                PropertySetters.SetProperties(instance, NativePropertyValue_slice.ToFacade(state));
                return NativeInstanceId_result.FromFacade(ObjectRegistry.Put(instance));
            }
            catch (Exception e)
            {
                HandleException(e);
                return NativeInstanceId_result.FromFacadeError(e.ToString());
            }
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
                _ => throw new Exception($"instance is not a NodeScript {instanceId.value}")
            };
        }

        private static void HandleException(Exception ex)
        {
            Console.WriteLine($"ERROR (FyroxLite): {ex}");
        }
    }
}