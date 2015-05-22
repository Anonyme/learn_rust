using System.Runtime.InteropServices;

namespace ClientDotNet
{
    class Program
    {
        [DllImport("embed.dll")]
        internal static extern void process();

        static void Main(string[] args)
        {
            process();
        }
    }
}
