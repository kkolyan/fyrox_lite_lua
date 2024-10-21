using System.Runtime.InteropServices;

namespace App01
{
    public static class BlittableHelper<T>
    {
        public static readonly bool IsBlittable;

        public static void Print() {
            var ty = typeof(T).FullName;
            var b = IsBlittable ? "" : " NOT";
            Console.WriteLine($"{ty} is{b} blittable");
        }

        static BlittableHelper()
        {
            try
            {
                // Class test
                if (default(T) != null)
                {
                    // Non-blittable types cannot allocate pinned handle
                    GCHandle.Alloc(default(T), GCHandleType.Pinned).Free();
                    IsBlittable = true;
                }
            }
            catch { }
        }
    }
}