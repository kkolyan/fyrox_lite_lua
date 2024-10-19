using System.Runtime.InteropServices;

namespace App01
{
    public partial class FyroxC
    {
        public static void HelloCSharp() {
            Console.WriteLine("I'm a Net Core");
        }

        [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf16, SetLastError = true)]
    
        public static partial void FyroxHello();
    }
}