import argparse

from asmwrapper.utils import formatHex
from asmwrapper.chunk import ChunkWrapper

FLAGS = [0, 0]


def main():
    # Handle Args
    parser = argparse.ArgumentParser(description='Assembly Wrapper Generator (Without jumps)')
    parser.add_argument(
        'file')
    parser.add_argument(
        '--output', '-o', help='Payload output location')
    args = parser.parse_args()

    # Initialize class
    wrapper = ChunkWrapper(FLAGS)

    # Parse shellcode file and scan for jumps
    wrapper.shellcodeReader(args.file)

    # Generate blocks and offset lists
    instructions = wrapper.generateLists()

    # Create random instruction path
    path = wrapper.createPath()

    # Combine payload
    payload = wrapper.generatePayload(instructions, path)

    if args.output is not None:
        try:
            open(args.output, 'wb').write(payload)
            print(f"\n[+] Written payload to {args.output}")
        except Exception as e:
            print(f"Exception occurred: {e}")

if __name__ == "__main__":
    main()


