// See https://aka.ms/new-console-template for more information
using System.Runtime.InteropServices;
using App01;

FyroxC.FyroxHello();
unsafe
{
    FyroxManualBindings.init_fyrox(new FyroxManualBindings.NativeScriptedApp
    {
        scripts = null,
        scripts_len = 0,
        functions = new FyroxManualBindings.NativeScriptAppFunctions
        {
            create_script_instance = Marshal.GetFunctionPointerForDelegate(FyroxImpls.CreateScriptInstance),
        }
    });
}
// BlittableHelper<short*>.Print();