#include <stdio.h>

// xxd -i output.bin
unsigned char output_bin[] = {
  0x55, 0x48, 0x89, 0xe5, 0x48, 0x81, 0xec, 0x20, 0x01, 0x00, 0x00, 0x48,
  0xb8, 0x49, 0x74, 0x27, 0x73, 0x20, 0x4d, 0x6f, 0x72, 0x48, 0xba, 0x62,
  0x69, 0x6e, 0x20, 0x54, 0x69, 0x6d, 0x65, 0x48, 0x89, 0x45, 0xd0, 0x48,
  0x89, 0x55, 0xd8, 0x66, 0xc7, 0x45, 0xe0, 0x21, 0x0a, 0xc7, 0x45, 0xf8,
  0x12, 0x00, 0x00, 0x00, 0x48, 0xb8, 0x52, 0x65, 0x6a, 0x6f, 0x69, 0x63,
  0x65, 0x2c, 0x48, 0xba, 0x20, 0x6d, 0x79, 0x20, 0x66, 0x65, 0x6c, 0x6c,
  0x48, 0x89, 0x45, 0xb0, 0x48, 0x89, 0x55, 0xb8, 0x48, 0xb8, 0x6f, 0x77,
  0x20, 0x4d, 0x6f, 0x72, 0x62, 0x73, 0x48, 0x89, 0x45, 0xc0, 0xc6, 0x45,
  0xc8, 0x0a, 0xc7, 0x45, 0xf4, 0x19, 0x00, 0x00, 0x00, 0x48, 0xb8, 0x54,
  0x69, 0x6d, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x48, 0xba, 0x67, 0x65, 0x74,
  0x20, 0x4d, 0x6f, 0x72, 0x62, 0x48, 0x89, 0x45, 0x90, 0x48, 0x89, 0x55,
  0x98, 0xc7, 0x45, 0xa0, 0x65, 0x64, 0x2c, 0x20, 0xc7, 0x45, 0xf0, 0x14,
  0x00, 0x00, 0x00, 0x48, 0xb8, 0x7b, 0x66, 0x7c, 0x6c, 0x54, 0x5b, 0x47,
  0x1e, 0x48, 0xba, 0x5c, 0x70, 0x1e, 0x5c, 0x70, 0x41, 0x1f, 0x5b, 0x48,
  0x89, 0x85, 0x60, 0xff, 0xff, 0xff, 0x48, 0x89, 0x95, 0x68, 0xff, 0xff,
  0xff, 0x48, 0xb8, 0x70, 0x5b, 0x47, 0x1c, 0x70, 0x4e, 0x4c, 0x18, 0x48,
  0xba, 0x5a, 0x1b, 0x43, 0x70, 0x49, 0x43, 0x1b, 0x48, 0x48, 0x89, 0x85,
  0x70, 0xff, 0xff, 0xff, 0x48, 0x89, 0x95, 0x78, 0xff, 0xff, 0xff, 0xc7,
  0x45, 0x80, 0x70, 0x43, 0x62, 0x1b, 0x66, 0xc7, 0x45, 0x84, 0x1f, 0x52,
  0xc7, 0x45, 0xec, 0x26, 0x00, 0x00, 0x00, 0xc7, 0x45, 0xe8, 0x2d, 0x00,
  0x00, 0x00, 0x48, 0xb8, 0x63, 0x74, 0x78, 0xa5, 0x8c, 0xa6, 0x56, 0x7e,
  0x48, 0xba, 0xd3, 0xff, 0x7b, 0xf4, 0x90, 0x40, 0x2e, 0x42, 0x48, 0x89,
  0x85, 0x30, 0xff, 0xff, 0xff, 0x48, 0x89, 0x95, 0x38, 0xff, 0xff, 0xff,
  0x48, 0xb8, 0x25, 0x7a, 0x49, 0xbd, 0x65, 0x52, 0x1f, 0x0b, 0x48, 0xba,
  0x20, 0xd4, 0xc3, 0xa6, 0x70, 0xaa, 0x12, 0x0e, 0x48, 0x89, 0x85, 0x40,
  0xff, 0xff, 0xff, 0x48, 0x89, 0x95, 0x48, 0xff, 0xff, 0xff, 0x48, 0xb8,
  0x6a, 0xb7, 0x6b, 0x72, 0xab, 0xc7, 0x05, 0x19, 0x48, 0xba, 0x25, 0x93,
  0xad, 0x9b, 0xa1, 0x4c, 0x8a, 0x10, 0x48, 0x89, 0x85, 0x50, 0xff, 0xff,
  0xff, 0x48, 0x89, 0x95, 0x58, 0xff, 0xff, 0xff, 0xc7, 0x45, 0xe4, 0x30,
  0x00, 0x00, 0x00, 0x8b, 0x45, 0xf8, 0x89, 0xc2, 0x48, 0x8d, 0x75, 0xd0,
  0xe8, 0xfb, 0x01, 0x00, 0x00, 0x48, 0xc7, 0xc2, 0x32, 0x00, 0x00, 0x00,
  0x48, 0x8d, 0xb5, 0xf0, 0xfe, 0xff, 0xff, 0x48, 0xc7, 0xc7, 0x02, 0x00,
  0x00, 0x00, 0x48, 0x31, 0xc0, 0x0f, 0x05, 0x48, 0x85, 0xc0, 0x7e, 0x5a,
  0xc7, 0x45, 0xfc, 0x00, 0x00, 0x00, 0x00, 0xeb, 0x22, 0x8b, 0x45, 0xfc,
  0x48, 0x98, 0x0f, 0xb6, 0x84, 0x05, 0xf0, 0xfe, 0xff, 0xff, 0x83, 0xf0,
  0x2f, 0x89, 0xc2, 0x8b, 0x45, 0xfc, 0x48, 0x98, 0x88, 0x94, 0x05, 0xf0,
  0xfe, 0xff, 0xff, 0x83, 0x45, 0xfc, 0x01, 0x83, 0x7d, 0xfc, 0x31, 0x7e,
  0xd8, 0x8b, 0x45, 0xec, 0x83, 0xc0, 0x01, 0x89, 0xc1, 0x48, 0x8d, 0xb5,
  0x60, 0xff, 0xff, 0xff, 0x48, 0x8d, 0xbd, 0xf0, 0xfe, 0xff, 0xff, 0xfc,
  0xf3, 0xa6, 0xe3, 0x1e, 0x8b, 0x45, 0xf4, 0x89, 0xc2, 0x48, 0x8d, 0x75,
  0xb0, 0xe8, 0x82, 0x01, 0x00, 0x00, 0x48, 0xc7, 0xc7, 0x00, 0x00, 0x00,
  0x00, 0x48, 0xc7, 0xc0, 0x3c, 0x00, 0x00, 0x00, 0x0f, 0x05, 0x8b, 0x45,
  0xe8, 0x83, 0xe8, 0x02, 0x48, 0x98, 0x0f, 0xb6, 0x84, 0x05, 0xf0, 0xfe,
  0xff, 0xff, 0x0f, 0xbe, 0xc0, 0xc1, 0xe0, 0x08, 0x89, 0xc2, 0x0f, 0xb6,
  0x85, 0xff, 0xfe, 0xff, 0xff, 0x83, 0xf0, 0x2f, 0x0f, 0xbe, 0xc0, 0x09,
  0xd0, 0x89, 0x85, 0xe0, 0xfe, 0xff, 0xff, 0x8b, 0x45, 0xe8, 0x83, 0xe8,
  0x04, 0x48, 0x98, 0x0f, 0xb6, 0x84, 0x05, 0xf0, 0xfe, 0xff, 0xff, 0x0f,
  0xbe, 0xc0, 0xc1, 0xe0, 0x08, 0x89, 0xc2, 0x0f, 0xb6, 0x85, 0xfd, 0xfe,
  0xff, 0xff, 0x83, 0xf0, 0x2f, 0x0f, 0xbe, 0xc0, 0x09, 0xd0, 0x89, 0x85,
  0xe4, 0xfe, 0xff, 0xff, 0x8b, 0x45, 0xe8, 0x83, 0xe8, 0x02, 0x48, 0x98,
  0x0f, 0xb6, 0x84, 0x05, 0xf0, 0xfe, 0xff, 0xff, 0x0f, 0xbe, 0xc0, 0x80,
  0xcc, 0x4a, 0x89, 0x85, 0xe8, 0xfe, 0xff, 0xff, 0x8b, 0x45, 0xe8, 0x83,
  0xe8, 0x06, 0x48, 0x98, 0x0f, 0xb6, 0x84, 0x05, 0xf0, 0xfe, 0xff, 0xff,
  0x0f, 0xbe, 0xc0, 0xc1, 0xe0, 0x08, 0x83, 0xc8, 0x32, 0x89, 0x85, 0xec,
  0xfe, 0xff, 0xff, 0x48, 0x8d, 0x85, 0x30, 0xff, 0xff, 0xff, 0x48, 0x83,
  0xc0, 0x10, 0x48, 0x8d, 0x95, 0xe0, 0xfe, 0xff, 0xff, 0x48, 0x89, 0xc6,
  0xbf, 0x20, 0x00, 0x00, 0x00, 0xe8, 0xcf, 0x00, 0x00, 0x00, 0x48, 0x8d,
  0x85, 0x30, 0xff, 0xff, 0xff, 0x48, 0x83, 0xc0, 0x08, 0x48, 0x8d, 0x95,
  0xe0, 0xfe, 0xff, 0xff, 0x48, 0x89, 0xc6, 0xbf, 0x20, 0x00, 0x00, 0x00,
  0xe8, 0xb0, 0x00, 0x00, 0x00, 0x48, 0x8d, 0x85, 0x30, 0xff, 0xff, 0xff,
  0x48, 0x83, 0xc0, 0x28, 0x48, 0x8d, 0x95, 0xe0, 0xfe, 0xff, 0xff, 0x48,
  0x89, 0xc6, 0xbf, 0x20, 0x00, 0x00, 0x00, 0xe8, 0x91, 0x00, 0x00, 0x00,
  0x48, 0x8d, 0x85, 0x30, 0xff, 0xff, 0xff, 0x48, 0x83, 0xc0, 0x18, 0x48,
  0x8d, 0x95, 0xe0, 0xfe, 0xff, 0xff, 0x48, 0x89, 0xc6, 0xbf, 0x20, 0x00,
  0x00, 0x00, 0xe8, 0x72, 0x00, 0x00, 0x00, 0x48, 0x8d, 0x95, 0xe0, 0xfe,
  0xff, 0xff, 0x48, 0x8d, 0x85, 0x30, 0xff, 0xff, 0xff, 0x48, 0x89, 0xc6,
  0xbf, 0x20, 0x00, 0x00, 0x00, 0xe8, 0x57, 0x00, 0x00, 0x00, 0x48, 0x8d,
  0x85, 0x30, 0xff, 0xff, 0xff, 0x48, 0x83, 0xc0, 0x20, 0x48, 0x8d, 0x95,
  0xe0, 0xfe, 0xff, 0xff, 0x48, 0x89, 0xc6, 0xbf, 0x20, 0x00, 0x00, 0x00,
  0xe8, 0x38, 0x00, 0x00, 0x00, 0x8b, 0x45, 0xf0, 0x89, 0xc2, 0x48, 0x8d,
  0x75, 0x90, 0xe8, 0x19, 0x00, 0x00, 0x00, 0x8b, 0x45, 0xe4, 0x89, 0xc2,
  0x48, 0x8d, 0xb5, 0x30, 0xff, 0xff, 0xff, 0xe8, 0x08, 0x00, 0x00, 0x00,
  0xe9, 0x81, 0xfe, 0xff, 0xff, 0x90, 0xc9, 0xc3, 0x48, 0xc7, 0xc7, 0x01,
  0x00, 0x00, 0x00, 0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, 0x0f, 0x05,
  0xc3, 0x55, 0x48, 0x89, 0xe5, 0x89, 0x7d, 0xfc, 0x48, 0x89, 0x75, 0xf0,
  0x48, 0x89, 0x55, 0xe8, 0x48, 0xbb, 0x89, 0x7d, 0xdc, 0x90, 0x90, 0x90,
  0xeb, 0x22, 0xeb, 0xf6, 0x48, 0xbb, 0x89, 0x45, 0xf8, 0x90, 0x90, 0x90,
  0xeb, 0x0c, 0x48, 0xbb, 0x8b, 0x40, 0x04, 0x89, 0x45, 0xf4, 0xeb, 0x28,
  0x48, 0xbb, 0x48, 0x8b, 0x45, 0xd0, 0x90, 0x90, 0xeb, 0xee, 0x48, 0xbb,
  0x48, 0x89, 0x75, 0xd0, 0x90, 0x90, 0xeb, 0x0c, 0x48, 0xbb, 0x48, 0x8b,
  0x45, 0xd0, 0x8b, 0x00, 0xeb, 0xd0, 0x48, 0xbb, 0x48, 0x89, 0x55, 0xc8,
  0x90, 0x90, 0xeb, 0xee, 0xc7, 0x45, 0xec, 0xb9, 0x79, 0x37, 0x9e, 0x48,
  0xbb, 0x13, 0xeb, 0x0a, 0xd2, 0xd0, 0xfa, 0x67, 0x44, 0xeb, 0xf7, 0xe8,
  0x48, 0xbb, 0x8b, 0x45, 0xec, 0x90, 0x90, 0x90, 0xeb, 0x02, 0x48, 0xbb,
  0x0f, 0xaf, 0x45, 0xdc, 0x90, 0x90, 0xeb, 0x02, 0x48, 0xbb, 0x89, 0x45,
  0xf0, 0x90, 0x90, 0x90, 0xeb, 0x00, 0xc7, 0x45, 0xfc, 0x00, 0x00, 0x00,
  0x00, 0x48, 0xbb, 0x1e, 0xa6, 0x28, 0x7b, 0x3a, 0xeb, 0x04, 0xd2, 0xeb,
  0xfb, 0xe8, 0xe9, 0xfe, 0x00, 0x00, 0x00, 0x48, 0xbb, 0x8b, 0x45, 0xf8,
  0xc1, 0xe0, 0x04, 0xeb, 0x37, 0xeb, 0xf6, 0x48, 0xbb, 0x8b, 0x45, 0xf8,
  0x8d, 0x0c, 0x02, 0xeb, 0x02, 0x48, 0xbb, 0x8b, 0x45, 0xf0, 0xc1, 0xe8,
  0x0b, 0xeb, 0x02, 0x48, 0xbb, 0x89, 0xc0, 0x83, 0xe0, 0x03, 0x90, 0xeb,
  0x00, 0x48, 0x8d, 0x14, 0x85, 0x00, 0x00, 0x00, 0x00, 0x48, 0xbb, 0x17,
  0x99, 0x31, 0xa8, 0x39, 0x6a, 0xeb, 0x19, 0xeb, 0xfc, 0xe8, 0x48, 0xbb,
  0x89, 0xc2, 0x8b, 0x45, 0xf8, 0x90, 0xeb, 0x02, 0x48, 0xbb, 0xc1, 0xe8,
  0x05, 0x31, 0xc2, 0x90, 0xeb, 0xbb, 0x48, 0xbb, 0x48, 0x8b, 0x45, 0xc8,
  0x90, 0x90, 0xeb, 0x0c, 0x48, 0xbb, 0x8b, 0x45, 0xec, 0x29, 0x45, 0xf0,
  0xeb, 0x16, 0x48, 0xbb, 0x48, 0x01, 0xd0, 0x8b, 0x10, 0x90, 0xeb, 0x02,
  0x48, 0xbb, 0x8b, 0x45, 0xf0, 0x01, 0xd0, 0x90, 0xeb, 0x16, 0x48, 0xbb,
  0x8b, 0x45, 0xf4, 0xc1, 0xe0, 0x04, 0xeb, 0x02, 0x48, 0xbb, 0x89, 0xc2,
  0x8b, 0x45, 0xf4, 0x90, 0xeb, 0x0c, 0x48, 0xbb, 0x31, 0xc8, 0x29, 0x45,
  0xf4, 0x90, 0xeb, 0xc6, 0x48, 0xbb, 0xc1, 0xe8, 0x05, 0x31, 0xc2, 0x90,
  0xeb, 0x3f, 0x48, 0xbb, 0x48, 0x01, 0xd0, 0x8b, 0x10, 0x90, 0xeb, 0x02,
  0x48, 0xbb, 0x8b, 0x45, 0xf0, 0x01, 0xd0, 0x90, 0xeb, 0x35, 0x48, 0x8d,
  0x14, 0x85, 0x00, 0x00, 0x00, 0x00, 0x48, 0xbb, 0xeb, 0x0b, 0x1f, 0x76,
  0xeb, 0xfc, 0xf8, 0xe1, 0xeb, 0xf6, 0xe8, 0x48, 0xbb, 0x48, 0x8b, 0x45,
  0xc8, 0x90, 0x90, 0xeb, 0xcf, 0x48, 0xbb, 0x8b, 0x45, 0xf0, 0x83, 0xe0,
  0x03, 0xeb, 0xd7, 0x48, 0xbb, 0x8b, 0x45, 0xf4, 0x8d, 0x0c, 0x02, 0xeb,
  0xee, 0x48, 0xbb, 0x31, 0xc8, 0x29, 0x45, 0xf8, 0x90, 0xeb, 0x02, 0x48,
  0xbb, 0x83, 0x45, 0xfc, 0x01, 0x90, 0x90, 0xeb, 0x00, 0x48, 0xbb, 0x8b,
  0x45, 0xfc, 0x3b, 0x45, 0xdc, 0xeb, 0x02, 0xeb, 0xf6, 0x0f, 0x82, 0xf0,
  0xfe, 0xff, 0xff, 0x48, 0xbb, 0x48, 0x8b, 0x45, 0xd0, 0x90, 0x90, 0xeb,
  0x04, 0xeb, 0xf6, 0x48, 0xbb, 0x8b, 0x55, 0xf8, 0x89, 0x10, 0x90, 0xeb,
  0x16, 0x48, 0xbb, 0x48, 0x8d, 0x50, 0x04, 0x90, 0x90, 0xeb, 0x02, 0x48,
  0xbb, 0x8b, 0x45, 0xf4, 0x89, 0x02, 0x90, 0xeb, 0x0c, 0x48, 0xbb, 0x48,
  0x8b, 0x45, 0xd0, 0x90, 0x90, 0xeb, 0xe4, 0x48, 0xbb, 0x5d, 0xc3, 0x90,
  0x90, 0x90, 0x90, 0xeb, 0x00, 0x90, 0x5d, 0xc3
};

int main() {
    (*(void (*)()) output_bin)();
    return 0;
}
