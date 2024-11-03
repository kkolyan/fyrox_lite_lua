// ReSharper disable InconsistentNaming
// ReSharper disable RedundantUnsafeContext
// ReSharper disable UnusedMember.Global
// ReSharper disable RedundantUsingDirective
using FyroxLite;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Collections;
namespace FyroxLite;

using System.Runtime.InteropServices;
using FyroxLite;

internal partial class FyroxNativeGlobal {
    

    [LibraryImport("../../../../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    internal static partial void init_fyrox(NativeScriptedApp app);

    internal delegate void NodeOnUpdate(NativeInstanceId thiz, float dt);

    internal delegate void NodeOnInit(NativeInstanceId thiz);

    internal delegate void NodeOnDeinit(NativeInstanceId thiz);

    internal delegate void NodeOnStart(NativeInstanceId thiz);

    internal delegate void NodeOnMessage(NativeInstanceId thiz, UserScriptMessage message);

    internal delegate void GameOnInit(NativeInstanceId thiz);

    internal delegate void GameOnUpdate(NativeInstanceId thiz);

    internal delegate NativeInstanceId_result CreateScriptInstance(NativeClassId thiz, NativePropertyValue_slice state);
}