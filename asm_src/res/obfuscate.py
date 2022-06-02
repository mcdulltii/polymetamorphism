import argparse

from asmwrapper.utils import formatHex
from asmwrapper.wrapper import AssemblyWrapper

FLAGS = [0, 0]


def main():
    # Handle Args
    parser = argparse.ArgumentParser(description='Assembly Wrapper Generator')
    parser.add_argument(
        'file')
    parser.add_argument(
        '--output', '-o', help='Payload output location')
    args = parser.parse_args()

    # Initialize class
    wrapper = AssemblyWrapper(FLAGS)

    # Create payload
    payload = wrapper.wrapShellcode(args.file)

    if args.output is not None:
        try:
            open(args.output, 'wb').write(payload)
            print(f"\n[+] Written payload to {args.output}")
        except Exception as e:
            print(f"Exception occurred: {e}")


if __name__ == "__main__":
    main()

